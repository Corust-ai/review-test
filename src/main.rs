mod order;
mod inventory;
mod notification;

use axum::{Router, routing::{get, post, put}};
use sqlx::PgPool;
use std::sync::Arc;
use tracing_subscriber;

pub struct AppState {
    pub db: PgPool,
    pub notification_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let notification_url = std::env::var("NOTIFICATION_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:8081".to_string());

    let pool = PgPool::connect(&db_url).await?;

    let state = Arc::new(AppState {
        db: pool,
        notification_url,
    });

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/orders", post(order::create_order))
        .route("/orders/{id}", get(order::get_order))
        .route("/orders/{id}/cancel", put(order::cancel_order))
        .route("/orders/{id}/ship", put(order::ship_order))
        .route("/inventory/{sku}", get(inventory::get_stock))
        .route("/inventory/{sku}/reserve", post(inventory::reserve_stock))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Order service listening on :3000");
    axum::serve(listener, app).await?;

    Ok(())
}
