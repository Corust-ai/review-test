mod bug_a;
mod user;
mod verify;

fn main() {
    println!("Review Test App");
    let avg = bug_a::average(&[]);
    println!("avg={}", avg);
}
