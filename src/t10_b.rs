// T10 b: atomic Ordering::Relaxed where SeqCst needed.
use std::sync::atomic::{AtomicBool, Ordering};
pub fn flag(a: &AtomicBool) -> bool {
    a.load(Ordering::Relaxed)
}
