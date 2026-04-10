use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct Cache {
    data: HashMap<String, (String, Instant)>,
    ttl: Duration,
    max_size: usize,
}

impl Cache {
    pub fn new(ttl_secs: u64, max_size: usize) -> Self {
        Self {
            data: HashMap::new(),
            ttl: Duration::from_secs(ttl_secs),
            max_size,
        }
    }

    /// BUG: division by zero when max_size is 0
    pub fn utilization(&self) -> f64 {
        self.data.len() as f64 / self.max_size as f64
    }

    /// BUG: unbounded growth, ignores max_size
    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key, (value, Instant::now()));
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        match self.data.get(key) {
            Some((val, created)) => {
                // BUG: returns expired entries
                Some(val.as_str())
            }
            None => None,
        }
    }

    /// BUG: holding Mutex across thread::sleep in async-like context
    pub fn background_cleanup(cache: Arc<Mutex<Cache>>) {
        thread::spawn(move || loop {
            let mut guard = cache.lock().unwrap();
            guard.data.retain(|_, (_, created)| {
                created.elapsed() < guard.ttl
            });
            // BUG: holds lock during sleep
            thread::sleep(Duration::from_secs(60));
        });
    }

    /// BUG: index out of bounds
    pub fn get_nth_key(&self, n: usize) -> &str {
        let keys: Vec<&String> = self.data.keys().collect();
        keys[n]  // no bounds check
    }
}
