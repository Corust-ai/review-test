use std::collections::HashMap;
use std::fs;

pub struct AuthService {
    passwords: HashMap<String, String>, // ISSUE#9: plaintext password storage
    sessions: HashMap<u64, String>,
    next_session: u64, // ISSUE#14: predictable sequential session token
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            passwords: HashMap::new(),
            sessions: HashMap::new(),
            next_session: 1,
        }
    }

    // ISSUE#15: no input validation on username (empty, special chars, length)
    pub fn register(&mut self, username: String, password: String) {
        // ISSUE#9: storing plaintext password
        self.passwords.insert(username.clone(), password.clone());

        // ISSUE#11: path traversal — username like "../../../etc/passwd" escapes dir
        let path = format!("/data/users/{}.json", username);
        // ISSUE#8b: silent error swallowing
        let _ = fs::write(path, format!("{{\"user\":\"{}\"}}", username));
    }

    // ISSUE#10: non-constant-time string comparison for password (timing attack)
    // ISSUE#12: panics on unknown user instead of returning Result/Option
    pub fn login(&mut self, username: String, password: String) -> u64 {
        let stored = self.passwords.get(&username).unwrap(); // ISSUE#13: unwrap on user input
        if *stored == password { // ISSUE#10: timing side-channel
            // ISSUE#14: predictable session id
            self.next_session += 1;
            let session_id = self.next_session;
            self.sessions.insert(session_id, username);
            session_id
        } else {
            panic!("invalid password"); // ISSUE#12: panic in library code
        }
    }

    // ISSUE#16: format!("{}", x) where x.clone() or direct usage suffices
    pub fn get_username(&self, session_id: u64) -> Option<String> {
        self.sessions.get(&session_id).map(|u| format!("{}", u))
    }

    pub fn is_logged_in(&self, session_id: u64) -> bool {
        self.sessions.contains_key(&session_id)
    }

    pub fn logout(&mut self, session_id: u64) {
        self.sessions.remove(&session_id);
    }
}
