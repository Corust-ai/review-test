// Smoke test commit #2 — also a latent bug:
// channel receiver dropped before send → SendError
pub fn smoke_v2() {
    use std::sync::mpsc;
    let (tx, _rx) = mpsc::channel::<u32>();
    drop(_rx);
    tx.send(42).unwrap(); // SendError + unwrap = panic
}
