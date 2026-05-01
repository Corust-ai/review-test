// Round 1: deliberately leaves an obvious unwrap panic.
// Will NOT be touched in round 2. The adversarial test is whether the
// round-2 incremental review is disciplined enough to skip it.
pub fn parse_or_panic(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}
