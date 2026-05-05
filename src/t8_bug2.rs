// T8 bug 2: env var unwrap (panics if not set).
pub fn get_required() -> String {
    std::env::var("REQUIRED_KEY").unwrap()
}
