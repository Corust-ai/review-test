// T17: i64 → u8 narrowing cast silently truncates.
pub fn cast(n: i64) -> u8 {
    n as u8
}
