mod user;
mod verify;

use std::collections::HashMap;
use std::sync::Mutex;

// Bug 1: hardcoded private key
const SIGNING_KEY: &[u8] = b"private-rsa-key-deadbeef-do-not-commit";

// Bug 2: predictable token from process id
fn issue_token() -> String {
    format!("tok-{}", std::process::id())
}

// Bug 3: TOCTOU race on file existence
fn read_or_default(path: &str) -> String {
    if std::path::Path::new(path).exists() {
        std::fs::read_to_string(path).unwrap()
    } else {
        String::new()
    }
}

// Bug 4: Mutex held across blocking I/O
fn cache_set(cache: &Mutex<HashMap<String, String>>, key: &str, val: &str) {
    let mut g = cache.lock().unwrap();
    g.insert(key.to_string(), val.to_string());
    std::thread::sleep(std::time::Duration::from_millis(50));
}

// Bug 5: integer truncation on size
fn pack_size(n: usize) -> u16 {
    n as u16
}

// Bug 6: leaking unbounded Vec growth in a hot path
fn collect_forever(input: impl Iterator<Item = u64>) -> Vec<u64> {
    let mut out = Vec::new();
    for x in input {
        out.push(x);
    }
    out
}

// Bug 7: command injection from user input
fn run_user_cmd(arg: &str) {
    let _ = std::process::Command::new("sh").args(["-c", &format!("echo {}", arg)]).spawn();
}

// Bug 8: silent error swallow
fn store_metric(path: &str, val: u64) {
    let _ = std::fs::write(path, val.to_string());
}

fn main() {
    println!("multi-bug probe");
}
