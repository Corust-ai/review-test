use axum::{extract::{Path, State}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use crate::AppState;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct StockInfo {
    pub sku: String,
    pub available: i32,
    pub reserved: i32,
}

#[derive(Deserialize)]
pub struct ReserveRequest {
    pub quantity: i32,
}

// ── Get Stock ──

pub async fn get_stock(
    State(state): State<Arc<AppState>>,
    Path(sku): Path<String>,
) -> Result<Json<StockInfo>, StatusCode> {
    // GOOD: parameterized query
    let stock = sqlx::query_as::<_, StockInfo>(
        "SELECT sku, available, reserved FROM inventory WHERE sku = $1"
    )
    .bind(&sku)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(stock))
}

// ── Reserve Stock (HTTP endpoint) ──

pub async fn reserve_stock(
    State(state): State<Arc<AppState>>,
    Path(sku): Path<String>,
    Json(req): Json<ReserveRequest>,
) -> Result<Json<StockInfo>, StatusCode> {
    // GOOD: validates positive quantity
    if req.quantity <= 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    reserve_stock_internal(&state.db, &sku, req.quantity)
        .await
        .map_err(|_| StatusCode::CONFLICT)?;

    get_stock_internal(&state.db, &sku)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

// ── Internal helpers ──

pub async fn reserve_stock_internal(
    pool: &PgPool, sku: &str, quantity: i32
) -> Result<(), sqlx::Error> {
    // BUG 7: TOCTOU race condition — check and update are not atomic.
    //        Two concurrent orders for the same SKU can both pass the check
    //        and over-reserve beyond available stock.
    let stock = sqlx::query_as::<_, StockInfo>(
        "SELECT sku, available, reserved FROM inventory WHERE sku = $1"
    )
    .bind(sku)
    .fetch_one(pool)
    .await?;

    if stock.available - stock.reserved < quantity {
        return Err(sqlx::Error::RowNotFound); // abuse error type as "insufficient stock"
    }

    sqlx::query("UPDATE inventory SET reserved = reserved + $1 WHERE sku = $2")
        .bind(quantity)
        .bind(sku)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn release_stock(
    pool: &PgPool, sku: &str, quantity: i32
) -> Result<(), sqlx::Error> {
    // GOOD: atomic decrement, no TOCTOU
    sqlx::query(
        "UPDATE inventory SET reserved = GREATEST(reserved - $1, 0) WHERE sku = $2"
    )
    .bind(quantity)
    .bind(sku)
    .execute(pool)
    .await?;

    Ok(())
}

async fn get_stock_internal(
    pool: &PgPool, sku: &str
) -> Result<StockInfo, sqlx::Error> {
    sqlx::query_as::<_, StockInfo>(
        "SELECT sku, available, reserved FROM inventory WHERE sku = $1"
    )
    .bind(sku)
    .fetch_one(pool)
    .await
}
