// T13 shared: drift edit (different impl, NO bug — non-PR-author change).
pub fn process(v: &[i32]) -> i32 {
    v.iter().copied().product()
}
