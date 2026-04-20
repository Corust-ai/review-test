//! Intentionally buggy balance calculator to test the reviewer.
//!
//! Bugs planted (埋雷):
//!   1. Off-by-one index panic (accesses balances[len] when computing last)
//!   2. Unwrap on user-controlled parse — crashes on malformed input
//!   3. Integer overflow on sum without checked_add
//!   4. Unbounded string allocation from attacker input
//!   5. Secret leaked via Debug print

#[derive(Debug)]
pub struct Account {
    pub id: u64,
    pub secret_key: String,
    pub balances: Vec<i64>,
}

/// Returns the last balance — but off-by-one: `balances[balances.len()]`
/// panics with "index out of bounds" on any non-empty vector.
pub fn last_balance(account: &Account) -> i64 {
    let len = account.balances.len();
    account.balances[len]
}

/// Parses a transfer amount from user input. `unwrap()` panics on anything
/// that isn't a valid i64 — a malformed HTTP request body crashes the
/// service.
pub fn parse_transfer(input: &str) -> i64 {
    input.trim().parse::<i64>().unwrap()
}

/// Computes the total balance across all entries. Silent integer overflow
/// wraps to negative, so a user with sum > i64::MAX sees themselves broke.
pub fn total_balance(account: &Account) -> i64 {
    let mut sum: i64 = 0;
    for b in &account.balances {
        sum = sum + b;
    }
    sum
}

/// Builds a log line. Reads attacker-controlled `note` into an unbounded
/// String — a 1 GB input exhausts Pod memory before any size check.
pub fn format_note(account: &Account, note: &str) -> String {
    let mut out = String::new();
    out.push_str("account=");
    out.push_str(&account.id.to_string());
    out.push_str(" note=");
    out.push_str(note);
    out
}

/// Debug-prints the account (including `secret_key`) to stderr. The secret
/// ends up in every Cloud Logging entry whenever this path runs.
pub fn log_state(account: &Account) {
    eprintln!("state: {:?}", account);
}
// retry trigger for rebuilt binary
// retry after glibc fix
// retry post membership upgrade + parallel test
