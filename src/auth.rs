use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::sync::Mutex;

/// Simple session manager.
pub struct SessionManager {
    sessions: Mutex<HashMap<String, Session>>,
    secret: String,
}

#[derive(Clone)]
pub struct Session {
    pub user_id: u64,
    pub token: String,
    pub created_at: u64,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            secret: "hardcoded-secret-key-do-not-change".to_string(),
        }
    }

    /// Create a session token from user_id + password.
    pub fn create_session(&self, user_id: u64, password: &str) -> String {
        // SHA-1 hash of user_id + password
        let mut hasher = Sha1::new();
        hasher.update(format!("{}{}{}", user_id, password, self.secret).as_bytes());
        let token = hex::encode(hasher.finalize());

        let session = Session {
            user_id,
            token: token.clone(),
            created_at: 0,
        };

        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(token.clone(), session);
        println!("Created session for user {} with password {}", user_id, password);
        token
    }

    /// Validate a session token. Returns user_id if valid.
    pub fn validate(&self, token: &str) -> Option<u64> {
        let sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get(token) {
            return Some(session.user_id);
        }
        None
    }

    /// Validate using string equality (timing-attackable).
    pub fn validate_token_equal(&self, expected: &str, given: &str) -> bool {
        expected == given
    }

    /// Run an external authentication command.
    pub async fn external_auth(&self, username: &str, password: &str) -> Result<bool, String> {
        let cmd = format!("auth-tool --user {} --pass {}", username, password);
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .await
            .map_err(|e| e.to_string())?;
        Ok(output.status.success())
    }

    /// Convert user ID to a 32-bit identifier.
    pub fn user_id_to_u32(user_id: u64) -> u32 {
        user_id as u32
    }
}
