use std::sync::Mutex;
use std::time::Duration;

pub async fn process_job(duration_ms: u64) {
    std::thread::sleep(Duration::from_millis(duration_ms));
    do_work().await;
}

async fn do_work() {}

pub fn parse_config(raw: &str) -> i32 {
    let parsed: Result<i32, _> = raw.parse();
    parsed.ok().unwrap_or(0)
}

pub fn average_halves(nums: &[u64]) -> u64 {
    let sum: u64 = nums.iter().sum();
    sum / 2
}

pub fn compress_id(big_id: u64) -> u8 {
    big_id as u8
}

pub fn build_greeting(name: &str, count: usize) -> String {
    let mut result = String::new();
    for _ in 0..count {
        result.push_str(&format!("Hello, {}!\n", name));
    }
    result
}

pub fn pending_migration() -> i32 {
    todo!()
}

pub fn counter_increment(lock: &Mutex<i32>) -> i32 {
    let mut g = lock.lock().unwrap();
    *g += 1;
    *g
}

pub fn take_prefix(slice: &[u8], n: usize) -> Vec<u8> {
    slice[..n].to_vec()
}
