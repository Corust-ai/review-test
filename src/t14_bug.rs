// T14: array-literal index OOB. PR-author bug.
pub fn pick() -> u32 {
    let arr = [1u32, 2, 3];
    arr[99]
}
