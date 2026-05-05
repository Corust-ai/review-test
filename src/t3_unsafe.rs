// T3 round 2: pointer arithmetic before deref.
pub fn read_raw(p: *const i32) -> i32 {
    unsafe { *p.wrapping_add(1) }
}
