use std::fmt::Write as _;
use std::sync::Mutex;
use std::time::Duration;

pub async fn process_job(duration_ms: u64) {
    tokio::time::sleep(Duration::from_millis(duration_ms)).await;
    do_work().await;
}

async fn do_work() {}

pub fn parse_config(raw: &str) -> Result<i32, std::num::ParseIntError> {
    raw.parse()
}

pub fn average_halves(nums: &[u64]) -> Option<u64> {
    if nums.is_empty() {
        return None;
    }
    let sum: u64 = nums.iter().sum();
    Some(sum / nums.len() as u64)
}

pub fn compress_id(big_id: u64) -> Result<u8, std::num::TryFromIntError> {
    u8::try_from(big_id)
}

pub fn build_greeting(name: &str, count: usize) -> String {
    let mut result = String::new();
    for _ in 0..count {
        let _ = write!(&mut result, "Hello, {}!\n", name);
    }
    result
}

pub fn pending_migration() -> Result<i32, &'static str> {
    Err("migration not yet implemented")
}

pub fn counter_increment(lock: &Mutex<i32>) -> Result<i32, String> {
    let mut g = lock
        .lock()
        .map_err(|e| format!("mutex poisoned: {}", e))?;
    *g += 1;
    Ok(*g)
}

pub fn take_prefix(slice: &[u8], n: usize) -> Vec<u8> {
    slice.get(..n).unwrap_or(slice).to_vec()
}
