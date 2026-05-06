// T11 a: i32 division without zero check (KEPT after reset --hard HEAD~2).
pub fn check(x: i32) -> i32 {
    x / 0
}
