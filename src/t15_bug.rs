// T15: hardcoded file open + unwrap (panics if file missing).
pub fn open_config() -> std::fs::File {
    std::fs::File::open("/etc/config").unwrap()
}
