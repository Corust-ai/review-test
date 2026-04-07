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

    /// Read a template file. Path is restricted to a fixed templates directory.
    pub async fn render_template(&self, template_name: &str, data: &str) -> Result<String, String> {
        // Reject path traversal attempts
        if template_name.contains("..") || template_name.contains('/') || template_name.contains('\\') {
            return Err("invalid template name".to_string());
        }
        let path = format!("templates/{}", template_name);
        let template = tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())?;
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
            // Compute and drop the guard BEFORE awaiting
            let cache_len = {
                let cache = self.cache.lock().unwrap();
                cache.len()
            };
            tokio::task::yield_now().await;
            results.push(chunk.len() + cache_len);
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

    /// Compute cache hit ratio. Returns 0.0 when total is 0.
    pub fn hit_ratio(hits: u64, total: u64) -> f64 {
        if total == 0 {
            return 0.0;
        }
        hits as f64 / total as f64
    }
}
