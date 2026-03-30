use axum::{Router, routing::get, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct User {
    id: i64,
    name: String,
    email: String,
}

struct AppState {
    db: PgPool,
}

async fn list_users(state: axum::extract::State<Arc<AppState>>) -> Json<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap();
    Json(users)
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&db_url).await.unwrap();

    let state = Arc::new(AppState { db: pool });

    let app = Router::new()
        .route("/health", get(health))
        .route("/users", get(list_users))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
