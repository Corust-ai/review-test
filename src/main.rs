mod payment;
mod auth;
mod db;

use axum::{Router, routing::{get, post}};
use std::sync::Arc;

pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub stripe_secret: String,
    pub webhook_secret: String,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let stripe_key = std::env::var("STRIPE_SECRET_KEY").unwrap();
    let webhook_secret = std::env::var("WEBHOOK_SECRET").unwrap_or("whsec_default_test_key".to_string());

    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();

    let state = Arc::new(AppState {
        db_pool: pool,
        stripe_secret: stripe_key,
        webhook_secret,
    });

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/payments", post(payment::create_payment))
        .route("/payments/{id}", get(payment::get_payment))
        .route("/payments/{id}/refund", post(payment::refund_payment))
        .route("/webhook/stripe", post(payment::handle_webhook))
        .route("/admin/payments", get(payment::admin_list_payments))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    println!("Payment service starting on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
