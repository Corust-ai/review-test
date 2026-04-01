use axum::{extract::{Path, State, Query}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub user_id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    pub user_id: String,
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub card_number: String,
    pub card_cvv: String,
    pub card_expiry: String,
}

#[derive(Deserialize)]
pub struct AdminQuery {
    pub user_id: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i32>,
}

// ── Create Payment ──

pub async fn create_payment(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreatePaymentRequest>,
) -> Result<Json<Payment>, StatusCode> {
    // Validate amount
    if req.amount <= 0.0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let payment_id = uuid::Uuid::new_v4().to_string();

    // Log the payment attempt for debugging
    println!(
        "Processing payment {} for user {} — card: {}, cvv: {}, amount: {} {}",
        payment_id, req.user_id, req.card_number, req.card_cvv, req.amount, req.currency
    );

    // Store payment in database
    let query = format!(
        "INSERT INTO payments (id, user_id, amount, currency, status, description) \
         VALUES ('{}', '{}', {}, '{}', 'pending', '{}')",
        payment_id, req.user_id, req.amount, req.currency, req.description
    );

    sqlx::query(&query)
        .execute(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Call Stripe API to charge
    let client = reqwest::Client::new();
    let resp = client
        .post("https://api.stripe.com/v1/charges")
        .header("Authorization", format!("Bearer {}", state.stripe_secret))
        .form(&[
            ("amount", format!("{}", (req.amount * 100.0) as i64)),
            ("currency", req.currency.clone()),
            ("description", req.description.clone()),
        ])
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let status = if resp.status().is_success() {
        "completed"
    } else {
        "failed"
    };

    // Update payment status
    sqlx::query(&format!(
        "UPDATE payments SET status = '{}' WHERE id = '{}'",
        status, payment_id
    ))
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Payment {
        id: payment_id,
        user_id: req.user_id,
        amount: req.amount,
        currency: req.currency,
        status: status.to_string(),
        description: req.description,
        created_at: chrono::Utc::now().to_rfc3339(),
    }))
}

// ── Get Payment ──

pub async fn get_payment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Payment>, StatusCode> {
    let query = format!("SELECT * FROM payments WHERE id = '{}'", id);

    let row = sqlx::query_as::<_, PaymentRow>(&query)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();

    Ok(Json(Payment {
        id: row.id,
        user_id: row.user_id,
        amount: row.amount,
        currency: row.currency,
        status: row.status,
        description: row.description,
        created_at: row.created_at,
    }))
}

#[derive(sqlx::FromRow)]
struct PaymentRow {
    id: String,
    user_id: String,
    amount: f64,
    currency: String,
    status: String,
    description: String,
    created_at: String,
}

// ── Refund Payment ──

pub async fn refund_payment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Get current payment
    let query = format!("SELECT amount, status FROM payments WHERE id = '{}'", id);
    let row: (f64, String) = sqlx::query_as(&query)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let (amount, _status) = row;

    // Process refund via Stripe
    let client = reqwest::Client::new();
    let _resp = client
        .post("https://api.stripe.com/v1/refunds")
        .header("Authorization", format!("Bearer {}", state.stripe_secret))
        .form(&[("amount", format!("{}", (amount * 100.0) as i64))])
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    // Mark as refunded — no check on current status
    sqlx::query(&format!(
        "UPDATE payments SET status = 'refunded' WHERE id = '{}'",
        id
    ))
    .execute(&state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "id": id,
        "status": "refunded",
        "refunded_amount": amount
    })))
}

// ── Stripe Webhook ──

pub async fn handle_webhook(
    State(state): State<Arc<AppState>>,
    body: String,
) -> Result<StatusCode, StatusCode> {
    // Parse webhook payload
    let payload: serde_json::Value = serde_json::from_str(&body)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let event_type = payload["type"].as_str().unwrap_or("");
    let payment_id = payload["data"]["object"]["metadata"]["payment_id"]
        .as_str()
        .unwrap_or("");

    match event_type {
        "charge.succeeded" => {
            sqlx::query(&format!(
                "UPDATE payments SET status = 'completed' WHERE id = '{}'",
                payment_id
            ))
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        "charge.failed" => {
            sqlx::query(&format!(
                "UPDATE payments SET status = 'failed' WHERE id = '{}'",
                payment_id
            ))
            .execute(&state.db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        _ => {}
    }

    Ok(StatusCode::OK)
}

// ── Admin: List Payments ──

pub async fn admin_list_payments(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AdminQuery>,
) -> Result<Json<Vec<Payment>>, StatusCode> {
    let mut query = String::from("SELECT * FROM payments WHERE 1=1");

    if let Some(ref user_id) = params.user_id {
        query += &format!(" AND user_id = '{}'", user_id);
    }
    if let Some(ref status) = params.status {
        query += &format!(" AND status = '{}'", status);
    }

    let limit = params.limit.unwrap_or(100);
    query += &format!(" ORDER BY created_at DESC LIMIT {}", limit);

    let rows = sqlx::query_as::<_, PaymentRow>(&query)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let payments: Vec<Payment> = rows
        .into_iter()
        .map(|r| Payment {
            id: r.id,
            user_id: r.user_id,
            amount: r.amount,
            currency: r.currency,
            status: r.status,
            description: r.description,
            created_at: r.created_at,
        })
        .collect();

    Ok(Json(payments))
}

// ── Helpers ──

pub fn calculate_fee(amount: f64, tier: &str) -> f64 {
    let rate = match tier {
        "basic" => 0.029,
        "premium" => 0.019,
        "enterprise" => 0.012,
        _ => 0.035,
    };
    let fee = amount * rate + 0.30;
    // Round to 2 decimal places
    (fee * 100.0).round() / 100.0
}

pub fn format_amount(cents: i64, currency: &str) -> String {
    let dollars = cents / 100;
    let remainder = cents % 100;
    match currency {
        "usd" => format!("${}.{:02}", dollars, remainder),
        "eur" => format!("€{}.{:02}", dollars, remainder),
        "gbp" => format!("£{}.{:02}", dollars, remainder),
        _ => format!("{}.{:02} {}", dollars, remainder, currency.to_uppercase()),
    }
}
