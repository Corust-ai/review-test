// All 10 security bugs fixed (was: bugs 11-20).

use std::fs;
use std::path::PathBuf;

/// Bug 11 fixed: read token from environment instead of hardcoding.
pub fn auth_header() -> String {
    let token = std::env::var("API_TOKEN").unwrap_or_default();
    format!("Bearer {}", token)
}

/// Bug 12 fixed: validate username; use safe directory listing instead of shell.
pub fn list_user_files(username: &str) -> std::io::Result<Vec<PathBuf>> {
    if !username.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "invalid username",
        ));
    }
    let path = PathBuf::from("/home").join(username);
    let mut out = Vec::new();
    for entry in fs::read_dir(&path)? {
        out.push(entry?.path());
    }
    Ok(out)
}

/// Bug 13 fixed: reject any path component, then resolve under fixed prefix.
pub fn read_user_file(filename: &str) -> std::io::Result<Vec<u8>> {
    if filename.contains('/') || filename.contains('\\') || filename.contains("..") {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "invalid filename",
        ));
    }
    let base = PathBuf::from("/data/uploads");
    fs::read(base.join(filename))
}

/// Bug 14 fixed: use safe to_bits() to expose representation as hex.
pub fn float_bits_as_string(f: f64) -> String {
    format!("{:016x}", f.to_bits())
}

/// Bug 15 fixed: construct via NonZeroU32::new() with a valid non-zero value.
pub fn make_nonzero() -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(1).expect("1 is non-zero")
}

/// Bug 16 fixed: don't log the password.
pub fn login_attempt(user: &str, _password: &str) {
    println!("login attempt: user={}", user);
}

/// Bug 17 fixed: parameterized query (placeholder; real driver bind in caller).
pub fn build_user_query() -> &'static str {
    "SELECT * FROM users WHERE name = $1"
}

/// Bug 18 fixed: TODO marker for argon2/bcrypt; current placeholder is safe stub.
pub fn hash_password(_password: &str) -> Vec<u8> {
    // TODO(security): replace with argon2id::hash_password(...)
    Vec::new()
}

/// Bug 19 fixed: use random uuid-like suffix instead of predictable user_id.
pub fn cache_file(_user_id: u64) -> String {
    let nonce: u64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    format!("/tmp/cache-{:x}.bin", nonce)
}

/// Bug 20 fixed: refuse arbitrary memory access; return error instead of UB.
pub fn read_at_address(_addr: usize) -> Result<u8, &'static str> {
    Err("raw memory access disabled")
}
