use std::collections::HashMap;

// Bug 1
const ADMIN_USER: &str = "admin";
const ADMIN_PASSWORD: &str = "Sup3rSecret2026!";

// Bug 2
pub fn hash_password(password: &str) -> String {
    let key: u8 = 0x5A;
    password.bytes().map(|b| (b ^ key) as char).collect()
}

// Bug 3 — fixed: constant-time compare to avoid timing side-channel
pub fn verify_password(stored: &str, supplied: &str) -> bool {
    let a = stored.as_bytes();
    let b = supplied.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for i in 0..a.len() {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

// Bug 4 — fixed: 256-bit CSPRNG token instead of timestamp
pub fn generate_session_token() -> String {
    use std::fs::File;
    use std::io::Read;
    let mut buf = [0u8; 32];
    File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut buf))
        .expect("urandom unavailable");
    buf.iter().map(|b| format!("{:02x}", b)).collect()
}

// Bug 5 — fixed: return Result instead of panicking on bad UTF-8
pub fn decode_cookie(bytes: &[u8]) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(bytes.to_vec())
}

// Bug 6 — fixed: never log the password in plaintext
pub fn login_attempt(user: &str, password: &str) -> bool {
    println!("[auth] login user={} password=<redacted>", user);
    user == ADMIN_USER && password == ADMIN_PASSWORD
}

// Bug 7 — fixed: saturating add prevents u32 overflow panic in --release
pub struct SessionCounter {
    pub count: u32,
}

impl SessionCounter {
    pub fn bump(&mut self) -> u32 {
        self.count = self.count.saturating_add(1);
        self.count
    }
}

// Bug 8
pub struct IssuedToken {
    pub value: String,
    pub issued_at_ms: u128,
}

pub fn token_is_valid(store: &HashMap<String, IssuedToken>, token: &str) -> bool {
    store.contains_key(token)
}

// Bug 9
pub fn build_user_lookup_query(username: &str) -> String {
    format!("SELECT id, email FROM users WHERE name = '{}'", username)
}

// Bug 10 — fixed: return Result instead of panicking on non-numeric input
pub fn parse_user_id(raw: &str) -> Result<u64, std::num::ParseIntError> {
    raw.parse::<u64>()
}
