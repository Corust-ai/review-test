pub fn buggy_func_3(input: &str) -> i64 {
    let val: i64 = input.parse().unwrap();
    let items = vec![1, 2, 3];
    items[val as usize] + val / (val - val)
}
