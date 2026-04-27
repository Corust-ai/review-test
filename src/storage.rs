use std::fs;
use std::path::{Path, PathBuf};

// Bug 11
pub fn read_user_file(base: &Path, user_path: &str) -> std::io::Result<String> {
    let joined = base.join(user_path);
    fs::read_to_string(joined)
}

// Bug 12
pub fn open_if_exists(path: &Path) -> Option<String> {
    if path.exists() {
        Some(fs::read_to_string(path).unwrap_or_default())
    } else {
        None
    }
}

// Bug 13
pub fn load_config(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

// Bug 14
pub fn read_upload(path: &Path) -> std::io::Result<Vec<u8>> {
    fs::read(path)
}

// Bug 15
pub fn save_audit_log(path: &Path, entry: &str) {
    let _ = fs::write(path, entry);
}

// Bug 16
pub fn average_file_size(sizes: &[u64]) -> u64 {
    let total: u64 = sizes.iter().sum();
    total / sizes.len() as u64
}

// Bug 17
pub fn slice_frame(buf: &[u8], offset: usize, len: usize) -> &[u8] {
    &buf[offset..offset + len]
}

// Bugs 18-20 fixed: removed.

pub fn storage_root() -> PathBuf {
    PathBuf::from("/var/lib/review-test")
}
