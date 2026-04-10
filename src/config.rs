use std::fs;
use std::path::Path;

pub struct AppConfig {
    pub db_url: String,
    pub api_key: String,
    pub max_retries: i32,
    pub timeout_ms: u64,
}

impl AppConfig {
    /// BUG: panics if file missing, leaks secrets in error
    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path)
            .expect(&format!("Failed to read config from {}", path));

        let mut db_url = String::new();
        let mut api_key = String::new();
        let mut max_retries: i32 = 3;
        let mut timeout_ms: u64 = 5000;

        for line in content.lines() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            // BUG: panics on malformed lines
            let key = parts[0].trim();
            let value = parts[1].trim();

            match key {
                "db_url" => db_url = value.to_string(),
                "api_key" => api_key = value.to_string(),
                // BUG: unwrap on parse
                "max_retries" => max_retries = value.parse().unwrap(),
                // BUG: integer overflow possible with negative values
                "timeout_ms" => timeout_ms = value.parse().unwrap(),
                _ => {} // silently ignore unknown keys
            }
        }

        Self { db_url, api_key, max_retries, timeout_ms }
    }

    /// BUG: writes secrets to world-readable file
    pub fn save(&self, path: &str) {
        let content = format!(
            "db_url={}\napi_key={}\nmax_retries={}\ntimeout_ms={}",
            self.db_url, self.api_key, self.max_retries, self.timeout_ms
        );
        fs::write(path, content).unwrap();
    }

    /// BUG: path traversal
    pub fn load_plugin(&self, name: &str) -> Vec<u8> {
        let plugin_path = format!("./plugins/{}", name);
        fs::read(&plugin_path).unwrap()
    }
}
