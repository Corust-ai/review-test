// Bug 07: unwrap on file read — panics if file is missing or unreadable.
pub fn read_config() -> String {
    std::fs::read_to_string("config.toml").unwrap()
}
