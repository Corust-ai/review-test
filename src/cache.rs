use std::collections::HashMap;

pub struct Cache<K, V> {
    map: HashMap<K, V>,
    capacity: usize,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> Cache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            capacity,
        }
    }

    // BUG A: panics on empty cache
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    // BUG B: divide by zero when capacity is 0 (Cache::new(0))
    pub fn fill_ratio(&self) -> usize {
        self.map.len() / self.capacity
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}
