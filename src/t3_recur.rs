// T3 commit A: unbounded recursion (no base case → stack overflow).
pub fn factorial(n: u64) -> u64 {
    n * factorial(n - 1)
}
