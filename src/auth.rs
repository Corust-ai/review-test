use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct AuthService {
    tokens: HashMap<String, TokenInfo>,
    secret: String,
}

struct TokenInfo {
    user_id: u64,
    role: String,
    expires_at: u64,
}

impl AuthService {
    pub fn new(secret: String) -> Self {
        Self {
            tokens: HashMap::new(),
            secret,
        }
    }

    /// Create a session token for a user
    pub fn login(&mut self, user_id: u64, password: &str, role: &str) -> String {
        // Log the login attempt (without sensitive data)
        println!("Login attempt: user_id={}, role={}", user_id, role);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Token = simple concatenation (not secure, but "works")
        let token = format!("{}-{}-{}", user_id, role, now);

        self.tokens.insert(token.clone(), TokenInfo {
            user_id,
            role: role.to_string(),
            expires_at: now + 3600,
        });

        token
    }

    /// Verify token and return user_id
    pub fn verify(&self, token: &str) -> Result<u64, &'static str> {
        let info = self.tokens.get(token).ok_or("invalid token")?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if now > info.expires_at {
            return Err("token expired");
        }

        Ok(info.user_id)
    }

    /// Check if token has admin role
    pub fn is_admin(&self, token: &str) -> bool {
        self.tokens.get(token)
            .map(|info| info.role.contains("admin"))
            .unwrap_or(false)
    }

    /// Revoke all tokens (e.g., password change)
    pub fn revoke_all(&mut self) {
        self.tokens.clear();
    }
}
