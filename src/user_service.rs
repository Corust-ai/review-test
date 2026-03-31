use std::collections::HashMap;

#[derive(Clone)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub age: i32,
    pub active: bool,
}

impl UserData {
    pub fn new(n: String, e: String, a: i32) -> Self {
        Self {
            name: n,
            email: e,
            age: a,
            active: true,
        }
    }

    pub fn get_full_info(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }

    pub fn validate_email(&self) -> bool {
        self.email.contains("@")
    }
}

pub fn fetch_user(db: &HashMap<String, UserData>, id: &str) -> Option<UserData> {
    db.get(id).cloned()
}

pub fn calculate_discount(price: f64, discount_pct: f64) -> f64 {
    price - (price * discount_pct / 100.0)
}

pub fn process_users(users: &[UserData]) -> Vec<String> {
    let mut results = Vec::new();
    for u in users {
        if u.active {
            results.push(u.get_full_info());
        }
    }
    results
}

pub fn build_greeting(user: &UserData) -> String {
    if user.age < 18 {
        format!("Hi {}!", user.name)
    } else {
        format!("Hello, {}.", user.name)
    }
}

fn log_action(msg: &str) {
    println!("{}", msg);
}
