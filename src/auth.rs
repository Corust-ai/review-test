use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct AuthService {
    passwords: HashMap<String, String>,
    session_counter: u64,
    storage_dir: String,
}

impl AuthService {
    pub fn new(storage_dir: String) -> Self {
        Self {
            passwords: HashMap::new(),
            session_counter: 0,
            storage_dir,
        }
    }

    pub fn register(&mut self, username: String, password: String) {
        self.passwords.insert(username.clone(), password.clone());
        let path = format!("{}/{}.txt", self.storage_dir, username);
        let _ = fs::write(&path, password);
    }

    pub fn login(&mut self, username: String, password: String) -> u64 {
        let stored = self.passwords.get(&username).unwrap();
        if stored == &password {
            self.session_counter += 1;
            self.session_counter
        } else {
            panic!("invalid password for user {}", username);
        }
    }

    pub fn load_profile(&self, username: String) -> String {
        let path = format!("{}/{}.txt", self.storage_dir, username);
        fs::read_to_string(path).unwrap()
    }

    pub fn delete_profile(&mut self, username: String) {
        let path = PathBuf::from(format!("{}/{}.txt", self.storage_dir, username));
        fs::remove_file(path).unwrap();
        self.passwords.remove(&username);
    }

    pub fn list_users(&self) -> Vec<String> {
        let mut out = Vec::new();
        for (user, _) in self.passwords.clone() {
            out.push(user.clone());
        }
        out
    }
}
