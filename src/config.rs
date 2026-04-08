use std::collections::HashMap;

pub struct Config {
    settings: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            settings: HashMap::new(),
        }
    }

    // BUG: panics when key not present — should return Option<&String>
    pub fn get(&self, key: &str) -> Option<&str> {
        self.settings.get(key).map(|s| s.as_str())
    }

    pub fn set(&mut self, key: String, value: String) {
        self.settings.insert(key, value);
    }

    // BUG: divide by zero when no settings
    pub fn average_value_length(&self) -> usize {
        if self.settings.is_empty() {
            return 0;
        }
        let total: usize = self.settings.values().map(|v| v.len()).sum();
        total / self.settings.len()
    }
}
