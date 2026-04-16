use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Cache {
    data: HashMap<String, (String, u64)>, // key → (value, expires_at)
    hits: i32,   // ISSUE#6: should be usize, i32 truncates on large caches
    misses: i32, // ISSUE#6b: same
}

impl Cache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    // ISSUE#1: key: String should be &str — only reads, never stores the key param itself
    pub fn get(&mut self, key: String) -> Option<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() // ISSUE#2: unwrap on system clock (can fail on some platforms)
            .as_secs();

        // ISSUE#3: unnecessary .clone() — could return reference
        match self.data.get(&key) {
            Some((val, expires)) => {
                // ISSUE#5: off-by-one: should be >= (expired AT the exact second)
                if now > *expires {
                    self.misses += 1;
                    None
                } else {
                    self.hits += 1;
                    Some(val.clone()) // ISSUE#3
                }
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }

    // ISSUE#1b: key and value by String, should be &str
    pub fn set(&mut self, key: String, value: String, ttl_secs: u64) {
        let expires = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + ttl_secs;
        self.data.insert(key, (value, expires));
    }

    // ISSUE#7: manual loop where .retain() would be idiomatic
    pub fn evict_expired(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut to_remove = Vec::new();
        for (k, (_, expires)) in &self.data {
            if now > *expires {
                to_remove.push(k.clone());
            }
        }
        for k in to_remove {
            self.data.remove(&k);
        }
    }

    // ISSUE#4: should use self.data.clear() instead of reassigning
    pub fn clear(&mut self) {
        self.data = HashMap::new();
        self.hits = 0;
        self.misses = 0;
    }

    // ISSUE#8: silent error swallowing with let _ = ...
    pub fn dump_to_file(&self, path: &str) {
        let mut lines = Vec::new();
        for (k, (v, _)) in &self.data {
            lines.push(format!("{}={}", k, v));
        }
        let _ = fs::write(path, lines.join("\n"));
    }

    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
}
