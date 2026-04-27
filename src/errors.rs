// 10 error-handling bugs (Bugs 31-33 fixed: removed).

#![allow(unused_imports)]
use std::process::Command;

/// Bug 34: unimplemented! in production.
pub fn handle_admin_action(action: &str) -> String {
    match action {
        "list" => "ok".to_string(),
        "promote" => unimplemented!("promote not done yet"),
        _ => "unknown".to_string(),
    }
}

/// Bug 35: ignored Command exit status.
pub fn run_migration() -> bool {
    let _ = Command::new("./migrate.sh").output();
    true
}

/// Bug 36: byte-indexing on user string — UTF-8 boundary panic.
pub fn first_n_bytes(s: &str, n: usize) -> &str {
    &s[..n]
}

/// Bug 37: misleading expect on fallible network parse.
pub fn parse_response_code(body: &str) -> u32 {
    body.split_whitespace()
        .next()
        .expect("response always has a status code")
        .parse()
        .expect("first token is always a number")
}

/// Bug 38: catch-all Err(_) swallowing all variants.
pub fn try_load(path: &str) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    }
}

/// Bug 39: todo!() in production.
pub fn calculate_tax(_amount: u64) -> u64 {
    todo!("waiting on finance team to confirm formula");
}

/// Bug 40: unwrap_or_default loses critical error info.
pub fn parse_balance(s: &str) -> i64 {
    s.parse::<i64>().unwrap_or_default()
}
