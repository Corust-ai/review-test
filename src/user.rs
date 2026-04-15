use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub role: String,
}

pub struct UserStore {
    users: HashMap<u64, User>,
    next_id: u64,
}

impl UserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, name: String, email: String, role: String) -> &User {
        let id = self.next_id;
        self.next_id += 1;
        let user = User { id, name, email, role };
        self.users.insert(id, user);
        self.users.get(&id).unwrap()
    }

    pub fn find_duplicate_emails(&self) -> Vec<String> {
        let mut duplicates = Vec::new();
        let users: Vec<&User> = self.users.values().collect();
        for i in 0..users.len() {
            for j in 0..users.len() {
                if i != j && users[i].email == users[j].email {
                    duplicates.push(users[i].email.clone());
                }
            }
        }
        duplicates
    }

    pub fn first_admin_email(&self) -> &str {
        for user in self.users.values() {
            if user.role == "admin" {
                let s = format!("{}", user.email);
                return Box::leak(s.into_boxed_str());
            }
        }
        ""
    }

    pub fn bulk_add(&mut self, entries: Vec<(String, String, String)>) -> Vec<u64> {
        let mut ids = Vec::new();
        for (name, email, role) in entries {
            let u = self.add(name, email, role);
            ids.push(u.id);
        }
        ids
    }

    pub fn get(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn list(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    pub fn delete(&mut self, id: u64) -> bool {
        self.users.remove(&id).is_some()
    }
}
