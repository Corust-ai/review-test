mod user;
mod verify;

fn risky(s: &str) -> u64 {
    s.parse::<u64>().unwrap()
}

fn main() {
    println!("{}", risky("nope"));
}
