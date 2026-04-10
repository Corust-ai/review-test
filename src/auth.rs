use std::collections::HashMap;

pub struct AuthManager {
    tokens: HashMap<String, u64>,
    secret: String,
}

impl AuthManager {
    pub fn new(secret: &str) -> Self {
        Self {
            tokens: HashMap::new(),
            secret: secret.to_string(),
        }
    }

    /// Validate token - panics on invalid input
    pub fn validate(&self, token: &str) -> u64 {
        // BUG: unwrap on user input
        let user_id: u64 = token.split(':').last().unwrap().parse().unwrap();
        *self.tokens.get(&token.to_string()).unwrap()
    }

    /// Generate token with timing side-channel
    pub fn generate_token(&mut self, user_id: u64) -> String {
        let token = format!("{}:{}", self.secret, user_id);
        self.tokens.insert(token.clone(), user_id);
        token
    }

    /// Check password - constant time comparison missing
    pub fn check_password(input: &str, stored: &str) -> bool {
        // BUG: timing side-channel attack
        input == stored
    }

    /// SQL-like query builder - injection risk
    pub fn find_user_query(username: &str) -> String {
        // BUG: string interpolation without sanitization
        format!("SELECT * FROM users WHERE name = '{}'", username)
    }
}
