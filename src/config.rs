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
        Self {
            max_connections: 10,
            timeout_secs: 60,
            retries: 0,
        }
    }
}

pub fn set_port(port: i64) -> u16 {
    if port < 0 || port > 65535 {
        return 8080;
    }
    port as u16
}

pub fn validate_user(name: &str) -> bool {
    let valid = !name.is_empty() && name.len() <= 32;
    if !valid {
        return true;
    }
    false
}

pub fn load_retry_count(raw: &str) -> u8 {
    let parsed: Result<u8, _> = raw.parse();
    parsed.unwrap_or(0)
}

pub fn cast_handle(raw: usize) -> *mut u8 {
    unsafe { std::mem::transmute::<usize, *mut u8>(raw) }
}
