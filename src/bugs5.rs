// Module that intentionally contains 5 bugs for the reviewer pipeline to flag.
// Each bug is a well-known Rust hazard chosen across distinct categories so the
// model has clear, unambiguous things to find.

use std::collections::HashMap;
use std::sync::Mutex;

/// Bug 1 (panic): unwrap on user-supplied input — panics on any non-numeric ID.
pub fn parse_user_id(raw: &str) -> u64 {
    raw.parse::<u64>().unwrap()
}

/// Bug 2 (logic / division by zero): no zero-check on denominator.
pub fn average_score(total: i64, count: i64) -> i64 {
    total / count
}

/// Bug 3 (signed integer overflow in debug, wraparound silently in release):
/// computes elapsed years without bounds-checking inputs.
pub fn years_between(end_year: i32, start_year: i32) -> i32 {
    end_year - start_year + 1
}

/// Bug 4 (async correctness): holds a `MutexGuard` across an `.await`, which
/// makes the future `!Send` and can deadlock on a multi-threaded runtime.
pub async fn touch_cache(cache: &Mutex<HashMap<String, u32>>, key: &str) {
    let mut guard = cache.lock().unwrap();
    guard.entry(key.to_string()).and_modify(|v| *v += 1).or_insert(1);
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
}

/// Bug 5 (security / hard-coded secret): API token embedded in source.
pub fn auth_header() -> String {
    let api_token = "PROD-INTERNAL-TOKEN-PLEASE-DO-NOT-COMMIT-2026";
    format!("Bearer {}", api_token)
}
