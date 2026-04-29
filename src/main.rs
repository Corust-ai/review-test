mod user;
mod verify;

fn unwrap_first<T: Clone>(v: &Vec<T>) -> T {
    v.first().unwrap().clone()
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let _ = unwrap_first(&v);
}
