// T9 bug 1: integer overflow with i32::MAX explicit.
pub fn add_max(x: i32) -> i32 {
    x + i32::MAX
}
