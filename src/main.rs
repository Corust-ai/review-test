mod user;
mod verify;

fn parse_id(raw: &str) -> u64 {
    raw.parse::<u64>().expect("must be u64")
}

fn first_byte(s: &str) -> u8 {
    s.as_bytes()[0]
}

fn shift(x: u32, n: u32) -> u32 {
    x << n
}

fn main() {
    let id = parse_id("not-a-number");
    let b = first_byte("");
    let s = shift(1, 64);
    println!("{} {} {}", id, b, s);
}
