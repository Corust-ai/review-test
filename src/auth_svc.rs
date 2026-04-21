use std::collections::HashMap;

// Bug 1: hardcoded admin credentials baked into binary
const ADMIN_USER: &str = "admin";
const ADMIN_PASSWORD: &str = "Sup3rSecret2026!";

// Bug 2: homegrown XOR "hash" — not a real one-way function
pub fn hash_password(password: &str) -> String {
    let key: u8 = 0x5A;
    password.bytes().map(|b| (b ^ key) as char).collect()
}

// Bug 3: password check using plain `==` — leaks length and prefix via timing
pub fn verify_password(stored: &str, supplied: &str) -> bool {
    stored == supplied
}

// Bug 4: session token is just the current unix millis — guessable, sequential
pub fn generate_session_token() -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    format!("tok-{}", ts)
}

// Bug 5: .unwrap() on from_utf8 panics on any non-UTF-8 body
pub fn decode_cookie(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

// Bug 6: password printed to logs in cleartext
pub fn login_attempt(user: &str, password: &str) -> bool {
    println!("[auth] login user={} password={}", user, password);
    user == ADMIN_USER && password == ADMIN_PASSWORD
}

// Bug 7: u32 session counter wraps silently at ~4B without overflow check
pub struct SessionCounter {
    pub count: u32,
}

impl SessionCounter {
    pub fn bump(&mut self) -> u32 {
        self.count = self.count + 1;
        self.count
    }
}

// Bug 8: token accepted forever — no expiry check anywhere
pub struct IssuedToken {
    pub value: String,
    pub issued_at_ms: u128,
}

pub fn token_is_valid(store: &HashMap<String, IssuedToken>, token: &str) -> bool {
    store.contains_key(token)
}

// Bug 9: query built by string concatenation — classic injection shape
pub fn build_user_lookup_query(username: &str) -> String {
    format!("SELECT id, email FROM users WHERE name = '{}'", username)
}

// Bug 10: .expect claiming invariant, but caller can pass any string
pub fn parse_user_id(raw: &str) -> u64 {
    raw.parse::<u64>().expect("user id must be numeric")
}
