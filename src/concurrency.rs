use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Bug 31 — MutexGuard held across .await
pub async fn update_cache(cache: Arc<Mutex<HashMap<String, u64>>>, key: String, val: u64) {
    let mut g = cache.lock().unwrap();
    g.insert(key.clone(), val);
    fetch_remote(&key).await;
}

async fn fetch_remote(_key: &str) -> u64 { 0 }

// Bug 32 — std::thread::sleep inside async fn
pub async fn rate_limited_call(req_id: u64) -> u64 {
    std::thread::sleep(std::time::Duration::from_millis(200));
    req_id
}

// Bug 33 — inconsistent lock order between functions (deadlock risk)
pub fn transfer(a: Arc<Mutex<u64>>, b: Arc<Mutex<u64>>, amount: u64) {
    let mut ga = a.lock().unwrap();
    let mut gb = b.lock().unwrap();
    *ga -= amount;
    *gb += amount;
}

pub fn refund(a: Arc<Mutex<u64>>, b: Arc<Mutex<u64>>, amount: u64) {
    let mut gb = b.lock().unwrap();
    let mut ga = a.lock().unwrap();
    *gb -= amount;
    *ga += amount;
}

// Bug 34 — tokio::spawn capturing non-Send (Rc)
pub async fn spawn_counter() {
    let counter = std::rc::Rc::new(std::cell::RefCell::new(0u64));
    tokio::spawn(async move {
        *counter.borrow_mut() += 1;
    });
}

// Bug 35 — JoinHandle dropped without await
pub async fn fire_and_forget(item: String) {
    tokio::spawn(async move {
        process(item).await;
    });
}

async fn process(_item: String) {}

// Bug 36 — cancellation-unsafe select! branch (mutates state before await)
pub async fn poll_two(a_ch: &mut tokio::sync::mpsc::Receiver<u64>, counter: &mut u64) {
    *counter += 1;
    tokio::select! {
        _ = a_ch.recv() => {},
        _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {},
    }
}

// Bug 37 — TOCTOU race on shared HashMap read+write
pub fn increment_visit(visits: &mut HashMap<String, u64>, ip: &str) {
    if visits.contains_key(ip) {
        let v = visits.get(ip).unwrap();
        visits.insert(ip.to_string(), v + 1);
    } else {
        visits.insert(ip.to_string(), 1);
    }
}

// Bug 38 — panic in Drop
pub struct AuditLogger {
    pub buf: Vec<u8>,
}

impl Drop for AuditLogger {
    fn drop(&mut self) {
        if self.buf.is_empty() {
            panic!("audit log dropped with empty buffer");
        }
    }
}

// Bug 39 — std::sync::Mutex held across await (compile-fragile pattern)
pub async fn cached_fetch(cache: Arc<Mutex<HashMap<String, String>>>, key: String) -> String {
    let g = cache.lock().unwrap();
    if let Some(v) = g.get(&key) {
        return v.clone();
    }
    let fetched = remote_lookup(&key).await;
    fetched
}

async fn remote_lookup(_k: &str) -> String { String::new() }

// Bug 40 — busy-loop without yield in async runtime
pub async fn wait_for_flag(flag: Arc<Mutex<bool>>) {
    loop {
        if *flag.lock().unwrap() {
            return;
        }
    }
}
