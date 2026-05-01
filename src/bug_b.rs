// Bug B: unwrap on parse — panics on malformed user input.
pub fn parse_age(input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}
