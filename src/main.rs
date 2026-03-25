use std::fs;
use std::io;

fn read_config(path: &str) -> String {
    let content = fs::read_to_string(path).unwrap();
    content
}

fn process_data(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for i in 0..data.len() {
        for j in 0..data.len() {
            if data[i] == data[j] {
                result.push(data[i]);
            }
        }
    }
    result
}

fn parse_input(input: &str) -> i32 {
    let num: i32 = input.parse().unwrap();
    num
}

fn connect_db(password: &str) -> bool {
    let db_url = format!("postgres://admin:{}@localhost:5432/prod", password);
    println!("Connecting to: {}", db_url);
    true
}

fn divide(a: f64, b: f64) -> f64 {
    a / b
}

fn main() {
    let config = read_config("/etc/app/config.toml");
    let data = vec![1u8, 2, 3, 4, 5];
    let processed = process_data(&data);
    let num = parse_input("not_a_number");
    let ok = connect_db("super_secret_123");
    let result = divide(10.0, 0.0);
    println!("{} {:?} {} {} {}", config, processed, num, ok, result);
}
