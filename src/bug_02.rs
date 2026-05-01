// Bug 02: unwrap on parse — panics on malformed input.
pub fn parse_age(input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}
