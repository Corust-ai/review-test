use std::collections::HashMap;

pub struct AuthService {
    password_hashes: HashMap<String, u64>,
    sessions: HashMap<u64, String>,
    next_session: u64,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            password_hashes: HashMap::new(),
            sessions: HashMap::new(),
            next_session: 1,
        }
    }

    pub fn register(&mut self, username: &str, password: &str) -> Result<(), String> {
        if username.is_empty() || username.len() > 64 {
            return Err("username must be 1-64 characters".to_string());
        }
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err("username must be alphanumeric or underscore".to_string());
        }
        let hash = simple_hash(password);
        self.password_hashes.insert(username.to_owned(), hash);
        Ok(())
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<u64, String> {
        let stored = self.password_hashes.get(username)
            .ok_or_else(|| "user not found".to_string())?;
        let input_hash = simple_hash(password);
        if *stored != input_hash {
            return Err("invalid password".to_string());
        }
        self.next_session = self.next_session.checked_add(1)
            .ok_or_else(|| "session counter overflow".to_string())?;
        let session_id = self.next_session;
        self.sessions.insert(session_id, username.to_owned());
        Ok(session_id)
    }

    pub fn get_username(&self, session_id: u64) -> Option<&str> {
        self.sessions.get(&session_id).map(|s| s.as_str())
    }

    pub fn is_logged_in(&self, session_id: u64) -> bool {
        self.sessions.contains_key(&session_id)
    }

    pub fn logout(&mut self, session_id: u64) {
        self.sessions.remove(&session_id);
    }
}

fn simple_hash(s: &str) -> u64 {
    let mut h: u64 = 5381;
    for b in s.bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u64);
    }
    h
}
