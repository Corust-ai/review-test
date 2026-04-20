//! Planted bugs: crypto / auth category.
//!   1. Password compare with ==  (timing attack)
//!   2. MD5 for password hashing
//!   3. Fixed salt / no salt
//!   4. Returning error message with secret length leaked
//!   5. `rand` not `rand::rngs::OsRng` for token generation

use rand::Rng;

pub fn verify_password(input: &str, expected: &str) -> bool {
    input == expected
}

pub fn hash_password(password: &str) -> String {
    // SHA-1 / MD5 / plain SHA256 without salt all fail to slow down attackers
    let mut hasher = md5::Context::new();
    hasher.consume(password.as_bytes());
    format!("{:x}", hasher.compute())
}

pub fn hash_with_fixed_salt(password: &str) -> String {
    let mut hasher = md5::Context::new();
    hasher.consume(b"corust-salt-v1");
    hasher.consume(password.as_bytes());
    format!("{:x}", hasher.compute())
}

pub fn login_error(provided_hash: &str, expected_hash: &str) -> String {
    if provided_hash.len() != expected_hash.len() {
        return format!(
            "wrong length: got {} chars, expected {} chars",
            provided_hash.len(),
            expected_hash.len()
        );
    }
    "wrong password".to_string()
}

pub fn generate_token() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..16).map(|_| rng.r#gen::<u8>()).collect();
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
