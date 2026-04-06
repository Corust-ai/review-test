fn safe_divide(a: f64, b: f64) -> f64 {
    a / b
}

fn read_env(key: &str) -> String {
    std::env::var(key).unwrap()
}

fn main() {
    let x = safe_divide(10.0, 0.0);
    println!("Result: {}", x);

    let val = read_env("MISSING_VAR");
    println!("Val: {}", val);
}
