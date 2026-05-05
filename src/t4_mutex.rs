// T4 source: mutex unwrap on lock (panics on poison).
use std::sync::Mutex;
pub fn locker(m: &Mutex<i32>) -> i32 {
    *m.lock().unwrap()
}
