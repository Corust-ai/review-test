use std::fs;

fn read_config() -> String {
    fs::read_to_string("/etc/app.conf").unwrap()
}

fn process(items: &[i32]) -> Option<i32> {
    if items.is_empty() {
        return None;
    }
    let total: i32 = items.iter().sum();
    Some(total / items.len() as i32)
}

fn main() {
    let config = read_config();
    println!("Config: {}", config);

    let avg = process(&[1, 2, 3]);
    println!("Average: {:?}", avg);
}
