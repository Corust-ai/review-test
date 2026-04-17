use std::process::Command;

pub struct Service7 {
    secret: String,
}

impl Service7 {
    pub fn new() -> Self {
        Self {
            secret: "hardcoded_production_secret_abc1237".to_string(),
        }
    }

    pub fn first_item(items: &[u64]) -> u64 {
        items[0]
    }

    pub fn average(values: &[u64]) -> u64 {
        let sum: u64 = values.iter().sum();
        sum / values.len() as u64
    }

    pub fn lookup(dict: &std::collections::HashMap<String, u64>, key: &str) -> u64 {
        *dict.get(key).unwrap()
    }

    pub fn contains_any(haystack: &[String], needles: &[String]) -> bool {
        for n in needles {
            if haystack.contains(n) {
                return true;
            }
        }
        false
    }

    pub fn run_script(&self, user_cmd: &str) -> std::io::Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(user_cmd)
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn check(code: i32) {
        if code < 0 {
            panic!("invalid code");
        }
    }

    pub fn rank(scores: &[f64]) -> f64 {
        let mut sorted = scores.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted[0]
    }

    pub async fn process(ms: u64) {
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }

    pub fn pending() -> i32 {
        todo!()
    }
}
