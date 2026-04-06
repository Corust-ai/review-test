use std::collections::HashMap;

fn get_user(db: &HashMap<String, String>, id: &str) -> String {
    db.get(id).unwrap().clone()
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn unused_function() {
    println!("this is never called");
}

fn main() {
    let mut db = HashMap::new();
    db.insert("1".to_string(), "Alice".to_string());

    let user = get_user(&db, "1");
    println!("User: {}", user);

    let result = divide(10, 0);
    println!("Result: {}", result);
}
