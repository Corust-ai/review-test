// 10 async/concurrency bugs (Bugs 21-23 fixed: removed).

#![allow(unused_imports)]
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;

/// Bug 24: JoinHandle dropped without await.
pub fn fire_and_forget(work: u64) {
    let _: JoinHandle<()> = tokio::spawn(async move {
        for _ in 0..work {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
    });
}

/// Bug 25: tokio::select! cancellation safety — counter mutated before await.
pub async fn racy_counter(counter: &mut u64, signal: tokio::sync::oneshot::Receiver<()>) {
    tokio::select! {
        _ = async {
            *counter += 1;
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        } => {}
        _ = signal => {}
    }
}

/// Bug 26: infinite async loop without yield.
pub async fn busy_loop() {
    loop {
        let _ = 1 + 1;
    }
}

/// Bug 27: blocking std::fs::read in async context.
pub async fn load_config() -> std::io::Result<Vec<u8>> {
    std::fs::read("/etc/app/config.json")
}

/// Bug 28: holding RwLockWriteGuard across .await.
pub async fn update_index(idx: &RwLock<HashMap<String, u32>>, key: &str) {
    let mut g = idx.write().unwrap();
    g.insert(key.to_string(), 1);
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
}

/// Bug 29: handles never awaited — task results lost.
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
    drop(handles);
}

/// Bug 30: tokio::time::interval drift.
pub async fn poll_every_second<F: Fn()>(f: F) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        f();
    }
}
