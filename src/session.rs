use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Session {
    pub user_id: u64,
    pub token: String,
    pub created_at: u64,
    pub ttl_seconds: u64,
}

pub struct SessionStore {
    sessions: Mutex<HashMap<String, Session>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    // FIXED: use duration_since(UNIX_EPOCH) and handle the Result properly
    pub fn create(&self, user_id: u64, token: String, ttl: u64) -> Session {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let session = Session {
            user_id,
            token: token.clone(),
            created_at: now,
            ttl_seconds: ttl,
        };
        self.sessions.lock().unwrap().insert(token, Session {
            user_id: session.user_id,
            token: session.token.clone(),
            created_at: session.created_at,
            ttl_seconds: session.ttl_seconds,
        });
        session
    }

    // BUG 2: holds MutexGuard across .await — would deadlock in async context
    //        (this fn isn't async, but the lock guard is held while doing slow I/O)
    pub fn validate(&self, token: &str) -> bool {
        let guard = self.sessions.lock().unwrap();
        if let Some(s) = guard.get(token) {
            // Slow synchronous network call while holding the lock
            let _ = std::net::TcpStream::connect("auth.example.com:443");
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            return now < s.created_at + s.ttl_seconds;
        }
        false
    }

    // BUG 3: integer overflow on TTL math — created_at + ttl_seconds may wrap
    pub fn expires_at(&self, token: &str) -> u64 {
        let guard = self.sessions.lock().unwrap();
        let s = guard.get(token).unwrap();
        s.created_at + s.ttl_seconds
    }

    // FIXED: return Option to let callers handle missing token
    pub fn get_user_id(&self, token: &str) -> Option<u64> {
        let guard = self.sessions.lock().unwrap();
        guard.get(token).map(|s| s.user_id)
    }

    // FIXED: guard against empty store before division
    pub fn average_ttl(&self) -> u64 {
        let guard = self.sessions.lock().unwrap();
        if guard.is_empty() {
            return 0;
        }
        let total: u64 = guard.values().map(|s| s.ttl_seconds).sum();
        total / guard.len() as u64
    }

    // FIXED: keep only sessions whose expiry is still in the future
    pub fn cleanup_expired(&self) -> usize {
        let mut guard = self.sessions.lock().unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let before = guard.len();
        guard.retain(|_, s| s.created_at.saturating_add(s.ttl_seconds) > now);
        before - guard.len()
    }
}
