// T2 commit A: silent integer truncation (u64 -> i32 cast).
pub fn shrink(big: u64) -> i32 {
    big as i32
}
