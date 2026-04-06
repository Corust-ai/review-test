fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn read_env(key: &str) -> String {
    std::env::var(key).unwrap()
}

fn main() {
    let x = safe_divide(10.0, 3.0);
    println!("Result: {:?}", x);

    let val = read_env("MISSING_VAR");
    println!("Val: {}", val);
}
