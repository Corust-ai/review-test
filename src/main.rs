mod bug_a;
mod bug_b;
mod user;
mod verify;

fn main() {
    println!("Review Test App");
    let avg = bug_a::average(&[]);
    println!("avg={}", avg);
    let age = bug_b::parse_age("not-a-number");
    println!("age={}", age);
}
