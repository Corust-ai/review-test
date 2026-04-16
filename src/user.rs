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

    pub fn get(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn list(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    pub fn delete(&mut self, id: u64) -> bool {
        self.users.remove(&id).is_some()
    }

    pub fn find_by_name(&self, name: String) -> Option<&User> {
        for user in self.users.values() {
            if user.name == name {
                return Some(user);
            }
        }
        None
    }

    pub fn admin_count(&self) -> usize {
        let mut count = 0;
        for user in self.users.values() {
            if user.role == "admin".to_string() {
                count += 1;
            }
        }
        count
    }
}
