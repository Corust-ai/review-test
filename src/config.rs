use std::collections::HashMap;

pub struct Config {
    values: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key.clone(), value.clone());
    }

    pub fn get(&self, key: String) -> String {
        self.values.get(&key).unwrap().clone()
    }

    pub fn get_int(&self, key: String) -> i64 {
        let v = self.values.get(&key).unwrap();
        v.parse::<i64>().unwrap()
    }
}
