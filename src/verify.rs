pub fn parse_u8(raw: &str) -> Result<u8, std::num::ParseIntError> {
    raw.parse()
}

pub fn first(v: &[i32]) -> Option<i32> {
    v.first().copied()
}
