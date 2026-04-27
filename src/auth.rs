use std::collections::HashMap;

// Bug 1
const ADMIN_USER: &str = "admin";
const ADMIN_PASSWORD: &str = "Sup3rSecret2026!";

// Bug 2
pub fn hash_password(password: &str) -> String {
    let key: u8 = 0x5A;
    password.bytes().map(|b| (b ^ key) as char).collect()
}

// Bug 3
pub fn verify_password(stored: &str, supplied: &str) -> bool {
    stored == supplied
}

// Bug 4
pub fn generate_session_token() -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("tok-{}", ts)
}

// Bug 5
pub fn decode_cookie(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

// Bug 6
pub fn login_attempt(user: &str, password: &str) -> bool {
    println!("[auth] login user={} password={}", user, password);
    user == ADMIN_USER && password == ADMIN_PASSWORD
}

// Bug 7
pub struct SessionCounter {
    pub count: u32,
}

impl SessionCounter {
    pub fn bump(&mut self) -> u32 {
        self.count = self.count + 1;
        self.count
    }
}

// Bugs 8-10 fixed: removed.
