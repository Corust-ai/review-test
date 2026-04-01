use axum::{extract::{Path, State}, http::StatusCode, Json};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::AppState;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: String,
    pub customer_id: String,
    pub items: serde_json::Value,
    pub total: Decimal,
    pub status: String,
    pub shipping_address: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub customer_id: String,
    pub items: Vec<OrderItem>,
    pub shipping_address: String,
    pub coupon_code: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct OrderItem {
    pub sku: String,
    pub quantity: i32,
    pub unit_price: Decimal,
}

// ── Create Order ──

pub async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<Json<Order>, StatusCode> {
    // Validate items
    if req.items.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // BUG 1: Integer overflow — quantity * unit_price with no overflow check,
    //        and negative quantities are accepted
    let total: Decimal = req.items.iter()
        .map(|item| item.unit_price * Decimal::from(item.quantity))
        .sum();

    // Apply coupon discount
    let total = if let Some(ref code) = req.coupon_code {
        apply_coupon(code, total)
    } else {
        total
    };

    let order_id = uuid::Uuid::new_v4().to_string();
    let items_json = serde_json::to_value(&req.items)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // BUG 2: SQL injection via shipping_address — format! instead of parameterized
    let query = format!(
        "INSERT INTO orders (id, customer_id, items, total, status, shipping_address) \
         VALUES ($1, $2, $3, $4, 'pending', '{}')",
        req.shipping_address
    );

    sqlx::query(&query)
        .bind(&order_id)
        .bind(&req.customer_id)
        .bind(&items_json)
        .bind(&total)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Reserve inventory for each item
    for item in &req.items {
        crate::inventory::reserve_stock_internal(&state.db, &item.sku, item.quantity).await
            .map_err(|_| StatusCode::CONFLICT)?;
    }

    // BUG 3: If inventory reservation fails for item N, items 0..N-1 are already
    //        reserved but never released — no rollback/compensation logic

    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
        .bind(&order_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Notify (fire-and-forget, OK to ignore errors)
    let _ = crate::notification::send_order_confirmation(&state.notification_url, &order).await;

    Ok(Json(order))
}

// ── Get Order ──

pub async fn get_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Order>, StatusCode> {
    // GOOD: parameterized query, proper error handling
    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(order))
}

// ── Cancel Order ──

pub async fn cancel_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Order>, StatusCode> {
    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
        .bind(&id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // GOOD: proper state machine check
    match order.status.as_str() {
        "pending" => {}
        "shipped" | "delivered" => return Err(StatusCode::CONFLICT),
        "cancelled" => return Ok(Json(order)),
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    sqlx::query("UPDATE orders SET status = 'cancelled' WHERE id = $1 AND status = 'pending'")
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Release reserved inventory
    if let Ok(items) = serde_json::from_value::<Vec<OrderItem>>(order.items.clone()) {
        for item in &items {
            let _ = crate::inventory::release_stock(&state.db, &item.sku, item.quantity).await;
        }
    }

    let updated = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated))
}

// ── Ship Order ──

pub async fn ship_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Order>, StatusCode> {
    // BUG 4: No status check — can ship a cancelled or already-shipped order
    sqlx::query("UPDATE orders SET status = 'shipped' WHERE id = $1")
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
        .bind(&id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // BUG 5: fetch_one after update — if id doesn't exist, the UPDATE silently
    //        affects 0 rows, then fetch_one panics with RowNotFound

    Ok(Json(order))
}

// ── Coupon Logic ──

fn apply_coupon(code: &str, total: Decimal) -> Decimal {
    // BUG 6: Coupon logic allows negative totals and has no validation
    match code {
        "HALF_OFF" => total / Decimal::from(2),
        "FLAT_50" => total - Decimal::from(50),   // can go negative
        _ => total,  // unknown codes silently ignored — not necessarily a bug
    }
}

// ── Helpers ──

/// Calculate estimated delivery days based on shipping tier.
/// This is pure computation — not blocking.
pub fn estimate_delivery(tier: &str) -> u32 {
    match tier {
        "express" => 2,
        "standard" => 5,
        "economy" => 10,
        _ => 7,
    }
}
