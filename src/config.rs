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
    pub fn get(&self, key: &str) -> &String {
        self.settings.get(key).unwrap()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.settings.insert(key, value);
    }

    // BUG: divide by zero when no settings
    pub fn average_value_length(&self) -> usize {
        let total: usize = self.settings.values().map(|v| v.len()).sum();
        total / self.settings.len()
    }
}
