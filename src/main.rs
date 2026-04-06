use std::fs;

fn load_config(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn compute_avg(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    Some(values.iter().sum::<f64>() / values.len() as f64)
}

fn main() {
    let config = load_config("/etc/app.toml");
    println!("{}", config);

    let avg = compute_avg(&[90.0, 80.0]);
    println!("{:?}", avg);
}
