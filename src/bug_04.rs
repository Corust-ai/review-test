// Bug 04: i32 overflow on sum (panics in debug, silently wraps in release).
pub fn sum_all(items: &[i32]) -> i32 {
    items.iter().sum()
}
