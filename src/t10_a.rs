// T10 a: transmute u32 to *const u8.
pub fn cast() -> *const u8 {
    let n: u32 = 0;
    unsafe { std::mem::transmute(n) }
}
