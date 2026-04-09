// Tiny parser for testing multi-turn conversation
pub fn parse_int(s: &str) -> i32 {
    s.parse().unwrap()
}

pub fn first_char(s: &str) -> char {
    s.chars().next().unwrap()
}
