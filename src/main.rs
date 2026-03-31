use std::collections::HashMap;
use std::io::Read;

fn process_user_input(input: &str) -> String {
    // Use parameterized query placeholder to prevent SQL injection
    let query = "SELECT * FROM users WHERE name = ?".to_string();
    let _ = input; // parameter would be bound separately
    query
}

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn parse_config(data: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in data.lines() {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    map
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn calculate_average(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len();
    Some(sum / count as f64)
}

fn get_element(v: &[i32], idx: usize) -> Option<i32> {
    v.get(idx).copied()
}

fn main() {
    let user_input = "admin'; DROP TABLE users; --";
    let query = process_user_input(user_input);
    println!("Query: {}", query);

    match divide(10, 2) {
        Some(result) => println!("Result: {}", result),
        None => println!("Error: division by zero"),
    }

    match calculate_average(&[1.0, 2.0, 3.0]) {
        Some(avg) => println!("Average: {}", avg),
        None => println!("Error: empty slice"),
    }

    match read_file("example.txt") {
        Ok(data) => println!("Data: {}", data),
        Err(e) => println!("Error reading file: {}", e),
    }
}
