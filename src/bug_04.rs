// Fixed: accumulate in i64 to avoid i32 overflow.
pub fn sum_all(items: &[i32]) -> i64 {
    items.iter().map(|&x| x as i64).sum()
}
