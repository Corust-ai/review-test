use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Cache {
    data: HashMap<String, (String, u64)>,
    hits: usize,   // FIXED#6: was i32
    misses: usize, // FIXED#6b
}

impl Cache {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    // FIXED#1: &str instead of String
    pub fn get(&mut self, key: &str) -> Option<&str> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0); // FIXED#2: graceful fallback instead of unwrap

        match self.data.get(key) {
            Some((val, expires)) => {
                // FIXED#5: >= instead of >
                if now >= *expires {
                    self.misses += 1;
                    None
                } else {
                    self.hits += 1;
                    Some(val.as_str()) // FIXED#3: return reference, no clone
                }
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }

    // FIXED#1b: &str params
    pub fn set(&mut self, key: &str, value: &str, ttl_secs: u64) {
        let expires = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
            + ttl_secs;
        self.data.insert(key.to_owned(), (value.to_owned(), expires));
    }

    // FIXED#7: use retain() instead of manual loop
    pub fn evict_expired(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.data.retain(|_, (_, expires)| now < *expires);
    }

    // FIXED#4: use .clear()
    pub fn clear(&mut self) {
        self.data.clear();
        self.hits = 0;
        self.misses = 0;
    }

    // FIXED#8: propagate error instead of silently swallowing
    pub fn dump_to_file(&self, path: &str) -> std::io::Result<()> {
        let mut lines = Vec::new();
        for (k, (v, _)) in &self.data {
            lines.push(format!("{}={}", k, v));
        }
        fs::write(path, lines.join("\n"))
    }

    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
}
