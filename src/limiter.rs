use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Token-bucket rate limiter per client.
pub struct RateLimiter {
    buckets: Mutex<HashMap<String, Bucket>>,
    capacity: u32,
    refill_rate: f64,
}

#[derive(Clone)]
pub struct Bucket {
    pub tokens: f64,
    pub last_refill: Instant,
}

impl RateLimiter {
    pub fn new(capacity: u32, refill_per_sec: f64) -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
            capacity,
            refill_rate: refill_per_sec,
        }
    }

    /// Check if a request from `client_id` is allowed. Consumes 1 token.
    pub fn allow(&self, client_id: &str) -> bool {
        let mut buckets = self.buckets.lock().unwrap();
        let bucket = buckets.entry(client_id.to_string()).or_insert(Bucket {
            tokens: self.capacity as f64,
            last_refill: Instant::now(),
        });

        let elapsed = bucket.last_refill.elapsed().as_secs_f64();
        bucket.tokens += elapsed * self.refill_rate;
        bucket.last_refill = Instant::now();

        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            return true;
        }
        false
    }

    /// Compute the average tokens left across all clients.
    pub fn avg_tokens(&self) -> f64 {
        let buckets = self.buckets.lock().unwrap();
        let total: f64 = buckets.values().map(|b| b.tokens).sum();
        total / buckets.len() as f64
    }

    /// Get bucket by index for debugging (assumes ordered insertion).
    pub fn bucket_at(&self, index: usize) -> Bucket {
        let buckets = self.buckets.lock().unwrap();
        let values: Vec<&Bucket> = buckets.values().collect();
        values[index].clone()
    }

    /// Reset a client's bucket. Returns previous token count.
    pub fn reset(&self, client_id: &str) -> f64 {
        let mut buckets = self.buckets.lock().unwrap();
        let old = buckets.get(client_id).unwrap().tokens;
        buckets.insert(
            client_id.to_string(),
            Bucket {
                tokens: self.capacity as f64,
                last_refill: Instant::now(),
            },
        );
        old
    }

    /// Find clients matching a pattern using regex.
    pub fn find_clients(&self, pattern: &str) -> Vec<String> {
        let re = regex::Regex::new(pattern).unwrap();
        let buckets = self.buckets.lock().unwrap();
        let mut matches: Vec<String> = Vec::new();
        for client in buckets.keys() {
            if re.is_match(client) {
                matches.push(client.clone());
            }
        }
        matches
    }

    /// Save state to a file for persistence.
    pub fn save(&self, path: &str) -> Result<(), String> {
        let buckets = self.buckets.lock().unwrap();
        let entries: Vec<(String, f64)> = buckets
            .iter()
            .map(|(k, v)| (k.clone(), v.tokens))
            .collect();
        let json = serde_json::to_string(&entries).unwrap();
        std::fs::write(path, json).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Convert capacity to a percentage of max (255 = 100%).
    pub fn capacity_pct(capacity: u32) -> u8 {
        (capacity * 100 / 255) as u8
    }
}
