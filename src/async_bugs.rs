// 10 async/concurrency bugs.

use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;

/// Bug 21: holds MutexGuard across .await — deadlock + !Send future.
pub async fn cache_touch(cache: &Mutex<HashMap<String, u32>>, key: &str) {
    let mut guard = cache.lock().unwrap();
    guard.entry(key.to_string()).and_modify(|v| *v += 1).or_insert(1);
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
}

/// Bug 22: blocking std::thread::sleep inside async fn — freezes runtime worker.
pub async fn delayed_response() -> &'static str {
    std::thread::sleep(std::time::Duration::from_secs(2));
    "done"
}

/// Bug 23: std::sync::Mutex sent across tokio::spawn boundary on multi-thread runtime.
pub fn spawn_with_sync_mutex(state: std::sync::Arc<Mutex<u64>>) {
    tokio::spawn(async move {
        let mut g = state.lock().unwrap();
        *g += 1;
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    });
}

/// Bug 24: JoinHandle dropped without await — task is silently detached.
pub fn fire_and_forget(work: u64) {
    let _: JoinHandle<()> = tokio::spawn(async move {
        for _ in 0..work {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
    });
}

/// Bug 25: tokio::select! cancellation safety — counter mutated before await,
/// state change sticks even if branch loses the race.
pub async fn racy_counter(counter: &mut u64, signal: tokio::sync::oneshot::Receiver<()>) {
    tokio::select! {
        _ = async {
            *counter += 1;
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        } => {}
        _ = signal => {}
    }
}

/// Bug 26: infinite async loop without yield — starves runtime worker.
pub async fn busy_loop() {
    loop {
        let _ = 1 + 1;
    }
}

/// Bug 27: blocking std::fs::read in async context — freezes worker thread.
pub async fn load_config() -> std::io::Result<Vec<u8>> {
    std::fs::read("/etc/app/config.json")
}

/// Bug 28: holding RwLockWriteGuard across .await — same hazard as MutexGuard.
pub async fn update_index(idx: &RwLock<HashMap<String, u32>>, key: &str) {
    let mut g = idx.write().unwrap();
    g.insert(key.to_string(), 1);
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
}

/// Bug 29: tokio::spawn capturing non-Send type (Rc) compiles only on
/// current_thread runtime; on multi-thread it would be a Send violation.
/// Even where it compiles, this is a foot-gun pattern.
pub async fn process_records(records: Vec<String>) {
    let total = std::sync::Arc::new(Mutex::new(0u32));
    let mut handles = vec![];
    for r in records {
        let total = total.clone();
        handles.push(tokio::spawn(async move {
            let mut g = total.lock().unwrap();
            *g += r.len() as u32;
        }));
    }
    // Bug: handles never awaited - results can be lost
    drop(handles);
}

/// Bug 30: tokio::time::interval drift — using `interval.tick().await` in
/// a tight loop without handling the case where work takes longer than the
/// interval period leads to bursts of catch-up calls.
pub async fn poll_every_second<F: Fn()>(f: F) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        f(); // if f() takes >1s, ticks pile up and fire back-to-back
    }
}
