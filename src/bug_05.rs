// Fixed: yield to the scheduler each iteration; cap total wait at 5 seconds.
use std::time::{Duration, Instant};

pub fn wait_until(cond: impl Fn() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if cond() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    false
}
