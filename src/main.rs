use std::collections::HashMap;
use std::io::Read;

fn process_user_input(input: &str) -> String {
    let query = format!("SELECT * FROM users WHERE name = '{}'", input);
    query
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn parse_config(data: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in data.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        map.insert(parts[0].to_string(), parts[1].to_string());
    }
    map
}

fn read_file(path: &str) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn calculate_average(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len();
    sum / count as f64
}

fn get_element(v: &[i32], idx: usize) -> i32 {
    v[idx]
}

fn main() {
    let user_input = "admin'; DROP TABLE users; --";
    let query = process_user_input(user_input);
    println!("Query: {}", query);

    let result = divide(10, 0);
    println!("Result: {}", result);

    let avg = calculate_average(&[]);
    println!("Average: {}", avg);

    let data = read_file("/etc/passwd");
    println!("Data: {}", data);
}
