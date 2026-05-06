// T28: infinite recursion (no base case).
pub fn rec(n: u32) -> u32 {
    rec(n) + 1
}
