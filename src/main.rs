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

fn connect_db(db_url: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if db_url.is_empty() {
        return Err("database URL is empty".into());
    }
    println!("Connecting to database...");
    Ok(true)
}

fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("division by zero")
    } else {
        Ok(a / b)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("config.toml")?;
    let data = vec![1u8, 2, 3, 4, 5];
    let processed = process_data(&data);
    let num = parse_input("42")?;
    let db_url = std::env::var("DATABASE_URL").unwrap_or_default();
    let ok = connect_db(&db_url)?;
    let result = divide(10.0, 3.0)?;
    println!("{} {:?} {} {} {}", config, processed, num, ok, result);
    Ok(())
}
