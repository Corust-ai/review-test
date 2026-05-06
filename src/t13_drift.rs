// T13 drift: BTreeMap unwrap on missing key — NOT PR author.
use std::collections::BTreeMap;
pub fn lookup(m: &BTreeMap<String, i32>, k: &str) -> i32 {
    *m.get(k).unwrap()
}
