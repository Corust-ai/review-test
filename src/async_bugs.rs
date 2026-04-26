// 6 async/concurrency bugs.

use std::collections::HashMap;
use std::sync::Mutex;
use tokio::task::JoinHandle;

/// Bug 13: holds MutexGuard across .await — deadlock + !Send future.
pub async fn cache_touch(cache: &Mutex<HashMap<String, u32>>, key: &str) {
    let mut guard = cache.lock().unwrap();
    guard.entry(key.to_string()).and_modify(|v| *v += 1).or_insert(1);
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
}

/// Bug 14: blocking std::thread::sleep inside async fn — freezes runtime worker.
pub async fn delayed_response() -> &'static str {
    std::thread::sleep(std::time::Duration::from_secs(2));
    "done"
}

/// Bug 15: std::sync::Mutex sent across tokio::spawn boundary on multi-thread runtime.
pub fn spawn_with_sync_mutex(state: std::sync::Arc<Mutex<u64>>) {
    tokio::spawn(async move {
        let mut g = state.lock().unwrap();
        *g += 1;
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    });
}

/// Bug 16: JoinHandle dropped without await — task is silently detached.
pub fn fire_and_forget(work: u64) {
    let _: JoinHandle<()> = tokio::spawn(async move {
        for _ in 0..work {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
    });
}

/// Bug 17: tokio::select! cancellation safety — counter mutated before await,
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

/// Bug 18: infinite async loop without yield — starves runtime worker.
pub async fn busy_loop() {
    loop {
        let _ = 1 + 1;
    }
}
