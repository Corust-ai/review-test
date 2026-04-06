use std::collections::HashMap;

fn get_value(map: &HashMap<String, i32>, key: &str) -> i32 {
    *map.get(key).unwrap()
}

fn average(nums: &[f64]) -> f64 {
    let sum: f64 = nums.iter().sum();
    sum / nums.len() as f64
}

fn main() {
    let mut scores = HashMap::new();
    scores.insert("alice".to_string(), 95);

    println!("Score: {}", get_value(&scores, "bob"));
    println!("Avg: {}", average(&[]));
}
