use std::collections::HashMap;

pub struct AuthService {
    passwords: HashMap<String, String>, // ISSUE#9 still: plaintext password storage
    sessions: HashMap<u64, String>,
    next_session: u64, // ISSUE#14 still: predictable sequential session token
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            passwords: HashMap::new(),
            sessions: HashMap::new(),
            next_session: 1,
        }
    }

    // ISSUE#15 still: no input validation on username
    pub fn register(&mut self, username: String, password: String) {
        // ISSUE#9 still: storing plaintext password
        self.passwords.insert(username.clone(), password);
        // FIXED#11: removed path traversal file write
        // FIXED#8b: removed silent error swallowing
    }

    // ISSUE#10 still: non-constant-time comparison
    // ISSUE#12 still: panics instead of Result
    pub fn login(&mut self, username: String, password: String) -> u64 {
        let stored = self.passwords.get(&username).unwrap(); // ISSUE#13 still: unwrap
        if *stored == password { // ISSUE#10 still
            // ISSUE#14 still: predictable
            self.next_session += 1;
            let session_id = self.next_session;
            self.sessions.insert(session_id, username);
            session_id
        } else {
            panic!("invalid password"); // ISSUE#12 still
        }
    }

    // FIXED#16: direct clone instead of format!
    pub fn get_username(&self, session_id: u64) -> Option<String> {
        self.sessions.get(&session_id).cloned()
    }

    pub fn is_logged_in(&self, session_id: u64) -> bool {
        self.sessions.contains_key(&session_id)
    }

    pub fn logout(&mut self, session_id: u64) {
        self.sessions.remove(&session_id);
    }
}
