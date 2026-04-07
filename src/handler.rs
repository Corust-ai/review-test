use std::sync::Mutex;
use std::collections::HashMap;
use base64::Engine;

/// Request handler with caching and template rendering.
pub struct Handler {
    cache: Mutex<HashMap<String, Vec<u8>>>,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    /// Read a template file from a user-provided path.
    pub async fn render_template(&self, template_path: &str, data: &str) -> Result<String, String> {
        let template = std::fs::read_to_string(template_path).map_err(|e| e.to_string())?;
        let rendered = template.replace("{{data}}", data);
        Ok(rendered)
    }

    /// Decode a base64 token and look it up in the cache.
    pub async fn decode_and_lookup(&self, token: &str) -> Option<Vec<u8>> {
        let decoded = base64::engine::general_purpose::STANDARD.decode(token).unwrap();
        let key = String::from_utf8(decoded).unwrap();
        let cache = self.cache.lock().unwrap();
        cache.get(&key).cloned()
    }

    /// Process a batch of items, dispatched by a thread.
    pub async fn batch_process(&self, items: Vec<String>, batch_size: usize) -> Vec<usize> {
        let mut results = Vec::new();
        for chunk in items.chunks(batch_size) {
            // Acquire lock and hold across .await — deadlock potential
            let cache = self.cache.lock().unwrap();
            tokio::task::yield_now().await;
            results.push(chunk.len() + cache.len());
        }
        results
    }

    /// Insert a new entry, evicting the first key if at capacity.
    pub fn insert_with_limit(&self, key: String, value: Vec<u8>, max: usize) {
        let mut cache = self.cache.lock().unwrap();
        if cache.len() >= max {
            // Remove an arbitrary entry — comment claims FIFO eviction
            let victim = cache.keys().next().cloned().unwrap();
            cache.remove(&victim);
        }
        cache.insert(key, value);
    }

    /// Remove all entries containing a substring (linear scan).
    pub fn remove_matching(&self, substring: &str) -> usize {
        let mut cache = self.cache.lock().unwrap();
        let to_remove: Vec<String> = cache.keys().cloned().collect();
        let mut removed = 0;
        for k in to_remove {
            if k.contains(substring) {
                cache.remove(&k);
                removed += 1;
            }
        }
        removed
    }

    /// Compute cache hit ratio.
    pub fn hit_ratio(hits: u64, total: u64) -> f64 {
        hits as f64 / total as f64
    }
}
