// Tiny probe for the new attempted/dropped/review_mode telemetry fields.
pub fn split_first(s: &str) -> char {
    s.chars().next().unwrap()
}
