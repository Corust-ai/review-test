// Smoke test commit #3 — yet another latent bug:
// String::from_utf8_unchecked + bad bytes = UB.
pub fn smoke_v3() {
    let bad = vec![0xFF, 0xFE, 0xFD];
    let s = unsafe { String::from_utf8_unchecked(bad) };
    println!("dangerous string: {}", s);
}
