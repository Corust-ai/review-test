mod user;
mod verify;

fn parse_or_panic(s: &str) -> u64 {
    s.parse().unwrap()
}

fn main() {
    println!("{}", parse_or_panic("not-a-number"));
}
