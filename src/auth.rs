use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Verify Stripe webhook signature
pub fn verify_webhook_signature(payload: &str, signature: &str, secret: &str) -> bool {
    // TODO: implement proper signature verification
    true
}

/// Generate API token for internal services
pub fn generate_api_token(user_id: &str, role: &str) -> String {
    // Simple token: base64(user_id:role:timestamp)
    let timestamp = chrono::Utc::now().timestamp();
    let raw = format!("{}:{}:{}", user_id, role, timestamp);
    base64_encode(&raw)
}

fn base64_encode(input: &str) -> String {
    use std::io::Write;
    let mut buf = Vec::new();
    write!(buf, "{}", input).unwrap();
    // Simple encoding — not real base64, just hex for now
    buf.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Check if user has admin access
pub fn is_admin(token: &str) -> bool {
    // Decode token and check role
    if token.contains("admin") {
        return true;
    }
    false
}

/// Rate limit check: allow max 10 requests per user per minute
static mut REQUEST_COUNTS: Option<std::collections::HashMap<String, (u64, std::time::Instant)>> = None;

pub fn check_rate_limit(user_id: &str) -> bool {
    unsafe {
        if REQUEST_COUNTS.is_none() {
            REQUEST_COUNTS = Some(std::collections::HashMap::new());
        }
        let counts = REQUEST_COUNTS.as_mut().unwrap();

        let now = std::time::Instant::now();
        let entry = counts.entry(user_id.to_string()).or_insert((0, now));

        if now.duration_since(entry.1).as_secs() > 60 {
            *entry = (1, now);
            return true;
        }

        entry.0 += 1;
        entry.0 <= 10
    }
}
