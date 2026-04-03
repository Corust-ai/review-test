use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Thread-safe in-memory cache with TTL expiration.
pub struct Cache<V: Clone> {
    store: Arc<RwLock<HashMap<String, CacheEntry<V>>>>,
    default_ttl: Duration,
}

struct CacheEntry<V> {
    value: V,
    expires_at: Instant,
}

impl<V: Clone + Send + Sync + 'static> Cache<V> {
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            default_ttl,
        }
    }

    /// Get a value if it exists and hasn't expired.
    pub fn get(&self, key: &str) -> Option<V> {
        let store = self.store.read().unwrap();
        match store.get(key) {
            Some(entry) if entry.expires_at > Instant::now() => Some(entry.value.clone()),
            Some(_) => None, // expired — will be cleaned up later
            None => None,
        }
    }

    /// Insert a value with the default TTL.
    pub fn set(&self, key: String, value: V) {
        self.set_with_ttl(key, value, self.default_ttl);
    }

    /// Insert a value with a custom TTL.
    pub fn set_with_ttl(&self, key: String, value: V, ttl: Duration) {
        let mut store = self.store.write().unwrap();
        store.insert(key, CacheEntry {
            value,
            expires_at: Instant::now() + ttl,
        });
    }

    /// Remove a specific key.
    pub fn invalidate(&self, key: &str) {
        let mut store = self.store.write().unwrap();
        store.remove(key);
    }

    /// Remove all expired entries. Call periodically.
    pub fn evict_expired(&self) {
        let mut store = self.store.write().unwrap();
        let now = Instant::now();
        store.retain(|_, entry| entry.expires_at > now);
    }

    /// Return the number of entries (including expired ones not yet evicted).
    pub fn len(&self) -> usize {
        self.store.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Spawn a background task that evicts expired entries periodically.
pub fn start_eviction_task<V: Clone + Send + Sync + 'static>(
    cache: Cache<V>,
    interval: Duration,
) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            cache.evict_expired();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let cache = Cache::new(Duration::from_secs(60));
        cache.set("key1".into(), "value1".to_string());
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("missing"), None);
    }

    #[test]
    fn test_expiration() {
        let cache = Cache::new(Duration::from_millis(1));
        cache.set("key1".into(), 42);
        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(cache.get("key1"), None);
    }

    #[test]
    fn test_invalidate() {
        let cache = Cache::new(Duration::from_secs(60));
        cache.set("key1".into(), "v".to_string());
        cache.invalidate("key1");
        assert_eq!(cache.get("key1"), None);
    }

    #[test]
    fn test_evict_expired() {
        let cache = Cache::new(Duration::from_millis(1));
        cache.set("a".into(), 1);
        cache.set("b".into(), 2);
        std::thread::sleep(Duration::from_millis(10));
        cache.set("c".into(), 3); // this one is fresh
        cache.evict_expired();
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get("c"), Some(3));
    }
}
