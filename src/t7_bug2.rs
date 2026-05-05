// T7 bug 2: Box::leak intentional memory leak.
pub fn make_static() -> &'static str {
    Box::leak(Box::new(String::from("hello"))).as_str()
}
