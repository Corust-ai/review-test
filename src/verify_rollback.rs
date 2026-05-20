// Quick rollback verification — intentional latent bug:
// Vec::set_len on uninitialized memory = undefined behavior.
pub fn rollback_smoke_test() {
    let mut v: Vec<u32> = Vec::with_capacity(10);
    unsafe { v.set_len(10); }
    println!("len after set_len: {}", v.len());
}
