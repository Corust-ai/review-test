use sqlx::PgPool;

/// Initialize database schema
pub async fn init_db(pool: &PgPool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS payments (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            amount FLOAT NOT NULL,
            currency TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            description TEXT,
            created_at TIMESTAMP DEFAULT NOW()
        )"
    )
    .execute(pool)
    .await
    .unwrap();
}

/// Delete all records — used for testing
pub async fn reset_db(pool: &PgPool) {
    sqlx::query("DROP TABLE IF EXISTS payments CASCADE")
        .execute(pool)
        .await
        .unwrap();
    init_db(pool).await;
}

/// Get total revenue
pub async fn get_total_revenue(pool: &PgPool) -> f64 {
    let row: (f64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(amount), 0) FROM payments WHERE status = 'completed'"
    )
    .fetch_one(pool)
    .await
    .unwrap();
    row.0
}

/// Export payments to CSV string
pub fn export_to_csv(payments: &[(String, String, f64, String)]) -> String {
    let mut csv = String::from("id,user_id,amount,status\n");
    for (id, user_id, amount, status) in payments {
        csv += &format!("{},{},{},{}\n", id, user_id, amount, status);
    }
    csv
}
