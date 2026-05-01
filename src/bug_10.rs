// Bug 10: unbounded allocation — `cap` comes from untrusted input and is not validated.
pub fn allocate_buffer(cap: usize) -> Vec<u8> {
    let mut buf = Vec::with_capacity(cap);
    buf.resize(cap, 0);
    buf
}
