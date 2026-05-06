// T21 placeholder — contains a latent bug (`*p` raw deref).
// PR will only chmod +x this file. bot must NOT review it as new.
pub unsafe fn deref(p: *const i32) -> i32 {
    *p
}
