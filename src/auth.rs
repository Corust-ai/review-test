use std::collections::HashMap;

pub struct AuthService {
    passwords: HashMap<String, String>, // ISSUE#9: plaintext password
    sessions: HashMap<u64, String>,
    next_session: u64,
}

impl AuthService {
    pub fn new() -> Self {
        Self { passwords: HashMap::new(), sessions: HashMap::new(), next_session: 1 }
    }

    pub fn register(&mut self, username: String, password: String) {
        self.passwords.insert(username, password); // ISSUE#9
    }

    // ISSUE#10: non-constant-time password compare
    pub fn login(&mut self, username: &str, password: &str) -> Option<u64> {
        match self.passwords.get(username) {
            Some(stored) if stored == password => { // ISSUE#10
                self.next_session += 1;
                self.sessions.insert(self.next_session, username.to_string());
                Some(self.next_session)
            }
            _ => None,
        }
    }
}
