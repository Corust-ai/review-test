// Bug 01: integer division by zero (will panic when denominator == 0).
pub fn safe_divide(numerator: i32, denominator: i32) -> i32 {
    numerator / denominator
}
