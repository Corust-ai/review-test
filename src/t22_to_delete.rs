// T22 placeholder — contains a latent bug (array index OOB).
// PR will delete this file. bot must NOT post phantom inline on
// the `-` lines (GitHub PR API can't anchor inline on deletions).
pub fn pick() -> i32 {
    let v = vec![1, 2, 3];
    v[99]
}
