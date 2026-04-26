// 6 security bugs — secrets, injection, UB.

use std::fs;
use std::process::Command;

/// Bug 7: hard-coded API token in source.
pub fn auth_header() -> String {
    let token = "PROD-INTERNAL-TOKEN-DO-NOT-COMMIT-2026";
    format!("Bearer {}", token)
}

/// Bug 8: command injection — user input concatenated into shell.
pub fn list_user_files(username: &str) -> std::io::Result<std::process::Output> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("ls /home/{}/", username))
        .output()
}

/// Bug 9: path traversal — user-controlled path with no validation.
pub fn read_user_file(filename: &str) -> std::io::Result<Vec<u8>> {
    fs::read(format!("/data/uploads/{}", filename))
}

/// Bug 10: unsafe transmute between unrelated types — instant UB.
pub fn float_bits_as_string(f: f64) -> String {
    let bytes: [u8; 8] = unsafe { std::mem::transmute(f) };
    String::from_utf8(bytes.to_vec()).unwrap_or_default()
}

/// Bug 11: mem::zeroed on non-zeroable type — instant UB.
pub fn make_nonzero() -> std::num::NonZeroU32 {
    unsafe { std::mem::zeroed() }
}

/// Bug 12: plaintext password logged.
pub fn login_attempt(user: &str, password: &str) {
    println!("login attempt: user={} password={}", user, password);
}
