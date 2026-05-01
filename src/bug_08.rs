// Bug 08: data race via static mut — concurrent calls produce undefined behavior.
static mut COUNTER: u32 = 0;

pub fn next_id() -> u32 {
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}
