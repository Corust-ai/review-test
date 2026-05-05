// T7-retest a (R2 amended): null pointer deref instead of unreachable_unchecked.
pub fn surprise(x: u32) -> u32 {
    if x > 100 {
        return x;
    }
    let p: *const u32 = std::ptr::null();
    unsafe { *p }
}
