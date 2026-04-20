//! Planted bugs: resource / DoS category.
//!   1. Unbounded recursion — stack overflow on adversarial input
//!   2. Reading entire file into memory before size check
//!   3. Quadratic string concatenation in a loop
//!   4. Sleep holding a Mutex → starves other threads
//!   5. String::from_utf8_unchecked on attacker bytes

use std::fs;
use std::io::Read;
use std::sync::Mutex;

pub fn count_depth(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::Array(a) => {
            1 + a.iter().map(count_depth).max().unwrap_or(0)
        }
        serde_json::Value::Object(m) => {
            1 + m.values().map(count_depth).max().unwrap_or(0)
        }
        _ => 0,
    }
}

pub fn read_file_then_check(path: &str, max_size: usize) -> std::io::Result<Vec<u8>> {
    let mut f = fs::File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    if buf.len() > max_size {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "too large",
        ));
    }
    Ok(buf)
}

pub fn build_log_line(entries: &[String]) -> String {
    let mut out = String::new();
    for e in entries {
        out = out + "\n" + e;
    }
    out
}

pub fn throttled_op(lock: &Mutex<u64>) -> u64 {
    let mut guard = lock.lock().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(5));
    *guard += 1;
    *guard
}

pub fn parse_name(bytes: Vec<u8>) -> String {
    unsafe { String::from_utf8_unchecked(bytes) }
}
