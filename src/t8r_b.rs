// T8-retest b: slice OOB index with literal beyond len.
pub fn pick(items: &[u32]) -> u32 {
    items[10]
}
