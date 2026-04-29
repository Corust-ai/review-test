mod user;
mod verify;

fn float_eq(a: f64, b: f64) -> bool {
    a == b
}

fn unsafe_cast(x: i32) -> u32 {
    x as u32
}

fn main() {
    println!("{} {}", float_eq(0.1 + 0.2, 0.3), unsafe_cast(-1));
}
