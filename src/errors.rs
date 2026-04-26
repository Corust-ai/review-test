// First 5 of 10 error-handling bugs fixed (bugs 31-35).
// Bugs 36-40 deliberately remain as-is.

use std::process::Command;

/// Bug 31 fixed: propagate parse error via Result.
pub fn fetch_record(id_str: &str) -> Result<String, std::num::ParseIntError> {
    let id: u64 = id_str.parse()?;
    Ok(format!("record-{}", id))
}

/// Bug 32 fixed: propagate I/O error.
pub fn save_state(state: &str) -> std::io::Result<()> {
    std::fs::write("/var/lib/app/state.txt", state)
}

/// Bug 33 fixed: log the parse failure instead of silently using 0.
pub fn parse_int_or_default(s: &str) -> i32 {
    match s.parse::<i32>() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("parse_int_or_default failed: {}", e);
            0
        }
    }
}

/// Bug 34 fixed: return error variant instead of panicking with unimplemented!.
pub fn handle_admin_action(action: &str) -> Result<String, &'static str> {
    match action {
        "list" => Ok("ok".to_string()),
        "promote" => Err("promote not implemented"),
        _ => Ok("unknown".to_string()),
    }
}

/// Bug 35 fixed: propagate I/O error AND check exit status.
pub fn run_migration() -> std::io::Result<bool> {
    let out = Command::new("./migrate.sh").output()?;
    Ok(out.status.success())
}

// ----- Bugs 36-40 still present below -----

/// Bug 36 (still buggy): byte-indexing on user string — panics on non-ASCII at the boundary.
pub fn first_n_bytes(s: &str, n: usize) -> &str {
    &s[..n]
}

/// Bug 37 (still buggy): misleading .expect on an obviously-fallible network parse.
pub fn parse_response_code(body: &str) -> u32 {
    body.split_whitespace()
        .next()
        .expect("response always has a status code")
        .parse()
        .expect("first token is always a number")
}

/// Bug 38 (still buggy): catch-all `Err(_)` swallowing all error variants without logging.
pub fn try_load(path: &str) -> Vec<u8> {
    match std::fs::read(path) {
        Ok(data) => data,
        Err(_) => Vec::new(),
    }
}

/// Bug 39 (still buggy): todo!() in a public production function.
pub fn calculate_tax(_amount: u64) -> u64 {
    todo!("waiting on finance team to confirm formula");
}

/// Bug 40 (still buggy): unwrap_or_default loses critical error info.
pub fn parse_balance(s: &str) -> i64 {
    s.parse::<i64>().unwrap_or_default()
}
