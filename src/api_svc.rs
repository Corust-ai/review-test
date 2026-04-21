use std::env;

// Bug 21: API signing secret hardcoded in source
pub const API_SECRET: &str = "api-shared-secret-do-not-commit";

// Bug 22: no length validation — attacker can send 1 GB username through
pub fn accept_username(raw: &str) -> String {
    raw.to_string()
}

// Bug 23: .unwrap() on required env var crashes service on missing config
pub fn db_url() -> String {
    env::var("DATABASE_URL").unwrap()
}

// Bug 24: error body leaks internal filesystem paths to the client
pub fn describe_error(err: &std::io::Error, path: &str) -> String {
    format!("failed to open {}: {}", path, err)
}

// Bug 25: i64 → u32 via `as` silently truncates — amount rolls over for >4B
pub fn to_wire_amount(amount_cents: i64) -> u32 {
    amount_cents as u32
}

// Bug 26: price * quantity with no checked_mul — overflows to small value on u64 max
pub fn total_cost(price_cents: u64, quantity: u64) -> u64 {
    price_cents * quantity
}

// Bug 27: `loop { queue.pop(); }` never exits if pop returns None forever
pub fn drain_queue(queue: &mut Vec<u32>) {
    loop {
        let _item = queue.pop();
    }
}

// Bug 28: Result dropped with `;` — every write silently fails
pub fn write_metric(path: &str, line: &str) {
    std::fs::write(path, line);
}

// Bug 29: user-controlled template passed to format-like writeln — log forgery
pub fn log_user_event(buf: &mut String, template: &str, name: &str) {
    buf.push_str(template);
    buf.push_str(name);
    buf.push('\n');
}

// Bug 30: header injection — user string dropped straight into HTTP response
pub fn build_cookie_header(session_id: &str) -> String {
    format!("Set-Cookie: session={}; HttpOnly", session_id)
}
