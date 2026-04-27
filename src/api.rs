use std::env;

// Bug 21
pub const API_SECRET: &str = "api-shared-secret-do-not-commit";

// Bug 22
pub fn accept_username(raw: &str) -> String {
    raw.to_string()
}

// Bug 23
pub fn db_url() -> String {
    env::var("DATABASE_URL").unwrap()
}

// Bug 24
pub fn describe_error(err: &std::io::Error, path: &str) -> String {
    format!("failed to open {}: {}", path, err)
}

// Bug 25
pub fn to_wire_amount(amount_cents: i64) -> u32 {
    amount_cents as u32
}

// Bug 26
pub fn total_cost(price_cents: u64, quantity: u64) -> u64 {
    price_cents * quantity
}

// Bug 27
pub fn drain_queue(queue: &mut Vec<u32>) {
    loop {
        let _item = queue.pop();
    }
}

// Bugs 28-30 fixed: removed.
