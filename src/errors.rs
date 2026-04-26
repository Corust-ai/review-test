// 10 error-handling bugs.

use std::process::Command;

/// Bug 31: unwrap on user-derived input in library function.
pub fn fetch_record(id_str: &str) -> String {
    let id: u64 = id_str.parse().unwrap();
    format!("record-{}", id)
}

/// Bug 32: swallowed Result via `let _ =`.
pub fn save_state(state: &str) {
    let _ = std::fs::write("/var/lib/app/state.txt", state);
}

/// Bug 33: .ok() discards meaningful error.
pub fn parse_int_or_default(s: &str) -> i32 {
    s.parse::<i32>().ok().unwrap_or(0)
}

/// Bug 34: unimplemented! left in production path.
pub fn handle_admin_action(action: &str) -> String {
    match action {
        "list" => "ok".to_string(),
        "promote" => unimplemented!("promote not done yet"),
        _ => "unknown".to_string(),
    }
}

/// Bug 35: ignored Command exit status — failure looks like success.
pub fn run_migration() -> bool {
    let _ = Command::new("./migrate.sh").output();
    true
}

/// Bug 36: byte-indexing on user string — panics on non-ASCII at the boundary.
pub fn first_n_bytes(s: &str, n: usize) -> &str {
    &s[..n]
}

/// Bug 37: misleading .expect on an obviously-fallible network parse.
pub fn parse_response_code(body: &str) -> u32 {
    body.split_whitespace()
        .next()
        .expect("response always has a status code")
        .parse()
        .expect("first token is always a number")
}

/// Bug 38: catch-all `Err(_)` swallowing all error variants without logging.
pub fn try_load(path: &str) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    }
}

/// Bug 39: todo!() in a public production function.
pub fn calculate_tax(amount: u64) -> u64 {
    todo!("waiting on finance team to confirm formula");
}

/// Bug 40: unwrap_or_default loses critical error info — silently returns
/// 0 on parse failure instead of propagating; downstream sees a "valid" 0.
pub fn parse_balance(s: &str) -> i64 {
    s.parse::<i64>().unwrap_or_default()
}
