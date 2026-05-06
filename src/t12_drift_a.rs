// T12 drift A: parse().unwrap() on user input — bug from "main drift", NOT PR author.
pub fn parse_age(input: &str) -> i32 {
    input.parse().unwrap()
}
