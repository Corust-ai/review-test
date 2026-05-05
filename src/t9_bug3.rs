// T9 bug 3: NonNull::dangling deref (UB).
use std::ptr::NonNull;
pub fn read_dangling() -> u8 {
    let p: NonNull<u8> = NonNull::dangling();
    unsafe { *p.as_ptr() }
}
