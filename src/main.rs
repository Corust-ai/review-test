use std::fs;

fn read_config() -> String {
    fs::read_to_string("/etc/app.conf").unwrap()
}

fn process(items: &[i32]) -> i32 {
    let total: i32 = items.iter().sum();
    total / items.len() as i32
}

fn main() {
    let config = read_config();
    println!("Config: {}", config);

    let avg = process(&[]);
    println!("Average: {}", avg);
}
