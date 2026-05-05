// T6 bug 3: SystemTime unwrap (panics if clock goes backwards).
use std::time::{SystemTime, UNIX_EPOCH};
pub fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
