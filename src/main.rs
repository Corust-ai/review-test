mod user;
mod verify;

fn unsafe_index(buf: &[u8], idx: i32) -> u8 {
    buf[idx as usize]
}

fn main() {
    println!("quota probe");
}
