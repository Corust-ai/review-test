// T6 bug 2: forget String (memory leak).
pub fn discard(s: String) {
    std::mem::forget(s);
}
