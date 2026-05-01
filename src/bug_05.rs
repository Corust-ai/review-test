// Bug 05: tight busy-loop with no sleep — burns 100% CPU.
pub fn wait_until(cond: impl Fn() -> bool) {
    while !cond() {}
}
