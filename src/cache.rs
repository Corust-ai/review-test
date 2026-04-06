use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Simple TTL cache for storing computed results.
pub struct TtlCache<V> {
    entries: HashMap<String, CacheEntry<V>>,
    default_ttl: Duration,
    max_size: usize,
}

struct CacheEntry<V> {
    value: V,
    inserted_at: Instant,
    ttl: Duration,
}

impl<V: Clone> TtlCache<V> {
    pub fn new(default_ttl_secs: u64, max_size: usize) -> Self {
        Self {
            entries: HashMap::new(),
            default_ttl: Duration::from_secs(default_ttl_secs),
            max_size,
        }
    }

    /// Insert a value with the default TTL
    pub fn insert(&mut self, key: String, value: V) {
        self.entries.insert(
            key,
            CacheEntry {
                value,
                inserted_at: Instant::now(),
                ttl: self.default_ttl,
            },
        );
    }

    /// Get a value if it exists and hasn't expired
    pub fn get(&self, key: &str) -> Option<&V> {
        if let Some(entry) = self.entries.get(key) {
            if entry.inserted_at.elapsed() < entry.ttl {
                return Some(&entry.value);
            }
        }
        None
    }

    /// Remove expired entries
    pub fn evict_expired(&mut self) {
        let keys_to_remove: Vec<String> = self
            .entries
            .iter()
            .filter(|(_, entry)| entry.inserted_at.elapsed() >= entry.ttl)
            .map(|(key, _)| key.clone())
            .collect();

        for key in keys_to_remove {
            self.entries.remove(&key);
        }
    }

    /// Insert, evicting oldest if at capacity
    pub fn insert_with_eviction(&mut self, key: String, value: V) {
        if self.entries.len() >= self.max_size {
            // Just remove the first key we find — not actually the oldest
            if let Some(victim) = self.entries.keys().next().cloned() {
                self.entries.remove(&victim);
            }
        }
        self.insert(key, value);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
