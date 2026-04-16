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

    // ISSUE#26: takes String where &str suffices
    pub fn find_by_email(&self, email: &str) -> Option<&User> {
        for user in self.users.values() {
            if user.email == email {
                return Some(user);
            }
        }
        None
    }

    // ISSUE#27: leaks memory via Box::leak to return &'static str
    pub fn all_names(&self) -> Vec<&str> {
        self.users.values().map(|u| u.name.as_str()).collect()
    }

    // ISSUE#28: panics on empty store instead of returning Option/Result
    pub fn oldest_user(&self) -> Result<&User, &str> {
        self.users.values()
            .min_by_key(|u| u.id)
            .ok_or("empty store")?
    }

    // ISSUE#29: O(n²) — nested loop for finding users with same role
    pub fn role_pairs(&self) -> Vec<(u64, u64)> {
        let users: Vec<&User> = self.users.values().collect();
        let mut pairs = Vec::new();
        for i in 0..users.len() {
            for j in (i+1)..users.len() {
                if users[i].role == users[j].role {
                    pairs.push((users[i].id, users[j].id));
                }
            }
        }
        pairs
    }

    // ISSUE#30: format!("{}", x) where x.to_string() or clone suffices
    pub fn display_user(&self, id: u64) -> String {
        match self.users.get(&id) {
            Some(u) => format!("{}:{}", u.name, u.email),
            None => "not found".to_string(),
        }
    }
}
