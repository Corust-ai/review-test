// T8 bug 3 (NEW in round 2): Vec::set_len out of bounds (UB).
pub fn dangerous(v: &mut Vec<i32>) {
    unsafe {
        v.set_len(v.capacity() + 100);
    }
}
