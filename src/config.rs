use std::num::ParseIntError;

pub struct Config {
    pub max_connections: u32,
    pub timeout_secs: u64,
    pub retries: u8,
}

impl Config {
    pub fn new() -> Self {
        Self {
            max_connections: 100,
            timeout_secs: 30,
            retries: 3,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

pub fn set_port(port: i64) -> Result<u16, String> {
    u16::try_from(port).map_err(|_| format!("port {} out of range (0..=65535)", port))
}

pub fn validate_user(name: &str) -> bool {
    !name.is_empty() && name.len() <= 32
}

pub fn load_retry_count(raw: &str) -> Result<u8, ParseIntError> {
    raw.parse()
}

pub fn cast_handle(raw: usize) -> *mut u8 {
    raw as *mut u8
}
