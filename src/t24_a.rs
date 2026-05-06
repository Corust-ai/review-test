// T24 commit 1: expect-on-Result panic.
pub fn parse_or_die(s: &str) -> u32 {
    s.parse().expect("must be valid")
}
