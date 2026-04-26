// 6 error-handling bugs.

use std::process::Command;

/// Bug 19: unwrap on user-derived input in library function.
pub fn fetch_record(id_str: &str) -> String {
    let id: u64 = id_str.parse().unwrap();
    format!("record-{}", id)
}

/// Bug 20: swallowed Result via `let _ =`.
pub fn save_state(state: &str) {
    let _ = std::fs::write("/var/lib/app/state.txt", state);
}

/// Bug 21: .ok() discards meaningful error.
pub fn parse_int_or_default(s: &str) -> i32 {
    s.parse::<i32>().ok().unwrap_or(0)
}

/// Bug 22: unimplemented! left in production path.
pub fn handle_admin_action(action: &str) -> String {
    match action {
        "list" => "ok".to_string(),
        "promote" => unimplemented!("promote not done yet"),
        _ => "unknown".to_string(),
    }
}

/// Bug 23: ignored Command exit status — failure looks like success.
pub fn run_migration() -> bool {
    let _ = Command::new("./migrate.sh").output();
    true
}

/// Bug 24: byte-indexing on user string — panics on non-ASCII at the boundary.
pub fn first_n_bytes(s: &str, n: usize) -> &str {
    &s[..n]
}
