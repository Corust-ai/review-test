use std::fs;
use std::path::{Path, PathBuf};

// Bug 11: path traversal — user-supplied path joined without sanitization
pub fn read_user_file(base: &Path, user_path: &str) -> std::io::Result<String> {
    let joined = base.join(user_path);
    fs::read_to_string(joined)
}

// Bug 12: TOCTOU — exists() then open(); attacker can swap file in between
pub fn open_if_exists(path: &Path) -> Option<String> {
    if path.exists() {
        Some(fs::read_to_string(path).unwrap_or_default())
    } else {
        None
    }
}

// Bug 13: .unwrap() on read_to_string — any I/O error panics the process
pub fn load_config(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

// Bug 14: unbounded read — attacker uploads 50 GB file, we try to hold it all
pub fn read_upload(path: &Path) -> std::io::Result<Vec<u8>> {
    fs::read(path)
}

// Bug 15: error swallowed — caller never learns the write failed
pub fn save_audit_log(path: &Path, entry: &str) {
    let _ = fs::write(path, entry);
}

// Bug 16: divide by zero when file list is empty
pub fn average_file_size(sizes: &[u64]) -> u64 {
    let total: u64 = sizes.iter().sum();
    total / sizes.len() as u64
}

// Bug 17: slice index with attacker-controlled offset/len, no bounds check
pub fn slice_frame(buf: &[u8], offset: usize, len: usize) -> &[u8] {
    &buf[offset..offset + len]
}

// Bug 18: JSON field extracted with .unwrap() — any malformed input panics
pub fn read_name_field(json: &str) -> String {
    let v: serde_json::Value = serde_json::from_str(json).unwrap();
    v["name"].as_str().unwrap().to_string()
}

// Bug 19: u64 → u32 truncation silently drops high bits on large files
pub fn file_size_as_u32(size: u64) -> u32 {
    size as u32
}

// Bug 20: non-atomic write — partial file visible to readers if we crash mid-write
pub fn overwrite_settings(path: &Path, body: &str) -> std::io::Result<()> {
    fs::write(path, body)?;
    Ok(())
}

pub fn storage_root() -> PathBuf {
    PathBuf::from("/var/lib/review-test")
}
