// 10 security bugs — secrets, injection, UB.

use std::fs;
use std::process::Command;

/// Bug 11: hard-coded API token in source.
pub fn auth_header() -> String {
    let token = "PROD-INTERNAL-TOKEN-DO-NOT-COMMIT-2026";
    format!("Bearer {}", token)
}

/// Bug 12: command injection — user input concatenated into shell.
pub fn list_user_files(username: &str) -> std::io::Result<std::process::Output> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("ls /home/{}/", username))
        .output()
}

/// Bug 13: path traversal — user-controlled path with no validation.
pub fn read_user_file(filename: &str) -> std::io::Result<Vec<u8>> {
    fs::read(format!("/data/uploads/{}", filename))
}

/// Bug 14: unsafe transmute between unrelated types — instant UB.
pub fn float_bits_as_string(f: f64) -> String {
    let bytes: [u8; 8] = unsafe { std::mem::transmute(f) };
    String::from_utf8(bytes.to_vec()).unwrap_or_default()
}

/// Bug 15: mem::zeroed on non-zeroable type — instant UB.
pub fn make_nonzero() -> std::num::NonZeroU32 {
    unsafe { std::mem::zeroed() }
}

/// Bug 16: plaintext password logged.
pub fn login_attempt(user: &str, password: &str) {
    println!("login attempt: user={} password={}", user, password);
}

/// Bug 17: SQL injection — user input concatenated into query.
pub fn build_user_query(username: &str) -> String {
    format!("SELECT * FROM users WHERE name = '{}'", username)
}

/// Bug 18: weak hashing — using a non-cryptographic / weak hash for password storage.
pub fn hash_password(password: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    password.hash(&mut hasher);
    hasher.finish()
}

/// Bug 19: predictable temp file path — race condition / overwrite attack.
pub fn cache_file(user_id: u64) -> String {
    format!("/tmp/cache-{}.bin", user_id)
}

/// Bug 20: unsafe raw pointer dereference of unvalidated address.
pub fn read_at_address(addr: usize) -> u8 {
    unsafe { *(addr as *const u8) }
}
