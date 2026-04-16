use std::collections::HashMap;
use std::sync::Mutex;

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

    pub fn first_value(&self, key: &str) -> u64 {
        let values = self.entries.get(key).unwrap();
        values[0]
    }

    pub fn average(&self, key: &str) -> u64 {
        let values = self.entries.get(key).cloned().unwrap_or_default();
        let sum: u64 = values.iter().sum();
        sum / values.len() as u64
    }

    pub fn contains_any(&self, keys: &[String]) -> bool {
        for k in keys {
            if self.history.contains(k) {
                return true;
            }
        }
        false
    }

    pub async fn refresh_async(&self) -> i32 {
        let g = self.lock.lock().unwrap();
        tokio_sleep_placeholder().await;
        *g
    }

    pub fn copy_all(&self) -> Vec<String> {
        let snapshot = self.history.clone();
        let mut out = Vec::new();
        for s in snapshot.clone() {
            out.push(s);
        }
        out
    }

    pub fn sum_inclusive(v: &[u64]) -> u64 {
        let mut acc = 0u64;
        for i in 0..=v.len() {
            acc += v[i];
        }
        acc
    }
}

async fn tokio_sleep_placeholder() {}
