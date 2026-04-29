mod user;
mod verify;

fn dangerous(s: &str) -> u64 {
    s.parse::<u64>().unwrap()
}

fn main() {
    let val = dangerous("not-a-number");
    println!("{}", val);
}
