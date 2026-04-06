use std::collections::HashMap;

fn get_value(map: &HashMap<String, i32>, key: &str) -> i32 {
    *map.get(key).unwrap()
}

fn average(nums: &[f64]) -> Option<f64> {
    if nums.is_empty() {
        return None;
    }
    Some(nums.iter().sum::<f64>() / nums.len() as f64)
}

fn main() {
    let mut scores = HashMap::new();
    scores.insert("alice".to_string(), 95);

    println!("Score: {}", get_value(&scores, "bob"));
    println!("Avg: {:?}", average(&[90.0, 85.0]));
}
