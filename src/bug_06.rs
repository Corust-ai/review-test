// Fixed: compare via tolerance instead of strict equality.
pub fn is_zero(x: f64) -> bool {
    x.abs() < f64::EPSILON
}
