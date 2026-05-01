// Smoke B: unwrap on file read.
pub fn read_secret() -> String {
    std::fs::read_to_string("/etc/secret").unwrap()
}
