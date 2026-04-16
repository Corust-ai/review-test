use std::collections::{HashMap, HashSet};
use tokio::sync::Mutex;

pub struct Cache {
    entries: HashMap<String, Vec<u64>>,
    history: Vec<String>,
    lock: Mutex<i32>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            history: Vec::new(),
            lock: Mutex::new(0),
        }
    }

    pub fn first_value(&self, key: &str) -> Option<u64> {
        self.entries.get(key).and_then(|v| v.first().copied())
    }

    pub fn average(&self, key: &str) -> Option<u64> {
        let values = self.entries.get(key)?;
        if values.is_empty() {
            return None;
        }
        let sum: u64 = values.iter().sum();
        Some(sum / values.len() as u64)
    }

    pub fn contains_any(&self, keys: &[String]) -> bool {
        let seen: HashSet<&String> = self.history.iter().collect();
        keys.iter().any(|k| seen.contains(k))
    }

    pub async fn refresh_async(&self) -> i32 {
        let g = self.lock.lock().await;
        let value = *g;
        drop(g);
        tokio_sleep_placeholder().await;
        value
    }

    pub fn copy_all(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn sum_inclusive(v: &[u64]) -> u64 {
        let mut acc = 0u64;
        for i in 0..v.len() {
            acc += v[i];
        }
        acc
    }
}

async fn tokio_sleep_placeholder() {}
