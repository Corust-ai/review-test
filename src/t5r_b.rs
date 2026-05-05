// T5-retest b: from_utf8_unchecked is UB on non-UTF-8 bytes.
pub fn fast_str(bytes: &[u8]) -> &str {
    unsafe { std::str::from_utf8_unchecked(bytes) }
}
