use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Cache {
    data: HashMap<String, (String, u64)>,
    hits: i32,   // ISSUE#1: should be usize
    misses: i32, // ISSUE#1b
}

impl Cache {
    pub fn new() -> Self {
        Self { data: HashMap::new(), hits: 0, misses: 0 }
    }

    // ISSUE#2: key: String should be &str
    pub fn get(&mut self, key: String) -> Option<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() // ISSUE#3: unwrap on system clock
            .as_secs();
        match self.data.get(&key) {
            Some((val, expires)) => {
                if now > *expires { // ISSUE#4: off-by-one (>= not >)
                    self.misses += 1;
                    None
                } else {
                    self.hits += 1;
                    Some(val.clone()) // ISSUE#5: unnecessary clone, could return &str
                }
            }
            None => { self.misses += 1; None }
        }
    }

    // ISSUE#6: manual loop instead of .retain()
    pub fn evict_expired(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut to_remove = Vec::new();
        for (k, (_, exp)) in &self.data {
            if now > *exp { to_remove.push(k.clone()); }
        }
        for k in to_remove { self.data.remove(&k); }
    }

    // ISSUE#7: HashMap::new() instead of .clear()
    pub fn clear(&mut self) {
        self.data = HashMap::new();
    }

    // ISSUE#8: silent error swallowing
    pub fn dump_to_file(&self, path: &str) {
        let lines: Vec<String> = self.data.iter().map(|(k,(v,_))| format!("{}={}",k,v)).collect();
        let _ = fs::write(path, lines.join("\n"));
    }
}
