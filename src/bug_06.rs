// Bug 06: floating-point equality comparison — unreliable due to rounding.
pub fn is_zero(x: f64) -> bool {
    x == 0.0
}
