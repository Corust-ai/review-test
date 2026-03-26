use std::collections::HashMap;

pub struct userData {
    pub Name: String,
    pub email: String,
    pub Age: i32,
    pub active: bool,
}

impl userData {
    pub fn new(n: String, e: String, a: i32) -> Self {
        Self {
            Name: n,
            email: e,
            Age: a,
            active: true,
        }
    }

    pub fn getFullInfo(&self) -> String {
        let mut s = String::new();
        s = s + &self.Name;
        s = s + " <";
        s = s + &self.email;
        s = s + ">";
        return s;
    }

    pub fn validate_email(&self) -> bool {
        if self.email.contains("@") {
            return true;
        } else {
            return false;
        }
    }
}

pub fn fetch_user(db: &HashMap<String, userData>, id: &str) -> userData {
    db.get(id).unwrap().clone()
}

pub fn calculate_discount(price: f64, discount_pct: f64) -> f64 {
    let result = price - (price * discount_pct / 100.0);
    return result;
}

pub fn process_users(users: &Vec<userData>) -> Vec<String> {
    let mut results = Vec::new();
    let mut i = 0;
    while i < users.len() {
        let u = &users[i];
        if u.active == true {
            let info = u.getFullInfo();
            results.push(info);
        }
        i = i + 1;
    }
    return results;
}

pub fn build_greeting(user: &userData) -> String {
    let greeting;
    if user.Age < 18 {
        greeting = format!("Hi {}!", user.Name);
    } else if user.Age >= 18 {
        greeting = format!("Hello, {}.", user.Name);
    } else {
        greeting = String::from("Welcome!");
    }
    greeting
}

fn log_action(msg: &str) {
    println!("{}", msg);
}
