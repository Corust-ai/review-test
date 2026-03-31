use std::fs;

fn read_config(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn process_data(data: &[u8]) -> Vec<u8> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for &byte in data {
        if seen.insert(byte) {
            result.push(byte);
        }
    }
    result
}

fn parse_input(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse()
}

fn connect_db(password: &str) -> bool {
    let _db_url = format!("postgres://admin:{}@localhost:5432/prod", password);
    println!("Connecting to database...");
    true
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("config.toml")?;
    let data = vec![1u8, 2, 3, 4, 5];
    let processed = process_data(&data);
    let num = parse_input("42")?;
    let ok = connect_db("super_secret_123");
    let result = divide(10.0, 3.0).unwrap_or(0.0);
    println!("{} {:?} {} {} {}", config, processed, num, ok, result);
    Ok(())
}
