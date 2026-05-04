// T1 commit B: unwrap-on-parse bug.
pub fn parse_int(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}
