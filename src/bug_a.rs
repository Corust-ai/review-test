// Bug A: integer division by zero (panics at runtime when divisor is 0).
pub fn divide(numerator: i32, denominator: i32) -> i32 {
    numerator / denominator
}

pub fn average(values: &[i32]) -> i32 {
    let sum: i32 = values.iter().sum();
    sum / values.len() as i32
}
