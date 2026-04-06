use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Thread-safe LRU cache with TTL support.
pub struct TaskCache {
    inner: Arc<Mutex<CacheInner>>,
}

struct CacheInner {
    entries: HashMap<String, CacheEntry>,
    max_size: usize,
}

struct CacheEntry {
    value: Vec<u8>,
    created_at: std::time::Instant,
    ttl_secs: u64,
}

impl TaskCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(CacheInner {
                entries: HashMap::new(),
                max_size,
            })),
        }
    }

    /// Get a value from the cache.
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let inner = self.inner.lock().unwrap();
        if let Some(entry) = inner.entries.get(key) {
            if entry.created_at.elapsed().as_secs() < entry.ttl_secs {
                return Some(entry.value.clone());
            }
        }
        None
    }

    /// Insert a value with TTL.
    pub fn insert(&self, key: String, value: Vec<u8>, ttl_secs: u64) {
        let mut inner = self.inner.lock().unwrap();
        if inner.entries.len() >= inner.max_size {
            // Evict oldest entry (LRU)
            if let Some(victim) = inner.entries.keys().next().cloned() {
                inner.entries.remove(&victim);
            }
        }
        inner.entries.insert(key, CacheEntry {
            value,
            created_at: std::time::Instant::now(),
            ttl_secs,
        });
    }

    /// Clear expired entries.
    pub fn evict_expired(&self) {
        let mut inner = self.inner.lock().unwrap();
        let expired: Vec<String> = inner.entries
            .iter()
            .filter(|(_, e)| e.created_at.elapsed().as_secs() >= e.ttl_secs)
            .map(|(k, _)| k.clone())
            .collect();
        for key in expired {
            inner.entries.remove(&key);
        }
    }

    /// Get cache stats as a formatted string.
    pub fn stats(&self) -> String {
        let inner = self.inner.lock().unwrap();
        let total_bytes: usize = inner.entries.values().map(|e| e.value.len()).sum();
        format!(
            "entries={}, bytes={}, max={}",
            inner.entries.len(),
            total_bytes,
            inner.max_size,
        )
    }
}

// Allow sharing across threads
unsafe impl Send for CacheInner {}
unsafe impl Sync for CacheInner {}
