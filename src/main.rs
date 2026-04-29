mod user;
mod verify;

fn read_first(buf: &[u8]) -> u8 {
    buf[0]
}

fn divide(a: u32, b: u32) -> u32 {
    a / b
}

fn main() {
    let r = divide(10, 0);
    let f = read_first(&[]);
    println!("{} {}", r, f);
}
