// 6 misc bugs — trait consistency, perf, lossy casts.

use std::collections::HashSet;

/// Bug 25: type implements Eq but contains f64 — partial_cmp can disagree
/// with ==, breaks HashMap/BTreeMap invariants.
#[derive(PartialEq, Eq, Hash)]
pub struct Measurement {
    pub label: String,
    pub value: f64,
}

/// Bug 26: manual Hash uses different fields than derived PartialEq —
/// `a == b` no longer implies `hash(a) == hash(b)`.
#[derive(PartialEq)]
pub struct UserKey {
    pub id: u64,
    pub name: String,
}

impl std::hash::Hash for UserKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Only hashes id, but PartialEq compares both fields → broken Hash contract.
        self.id.hash(state);
    }
}

/// Bug 27: Vec::contains in a tight loop — O(n*m). Should use HashSet.
pub fn dedupe_against_blocklist(items: Vec<String>, blocklist: Vec<String>) -> Vec<String> {
    items.into_iter().filter(|x| !blocklist.contains(x)).collect()
}

/// Bug 28: lossy cast i64 → u32 — silent truncation on values > u32::MAX.
pub fn id_as_u32(id: i64) -> u32 {
    id as u32
}

/// Bug 29: clone of large struct in hot loop — ~O(n) extra allocations.
pub fn prefix_sums(buf: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut out = Vec::new();
    let mut acc = 0;
    for item in buf {
        let copy = item.clone();
        acc += copy.len();
        out.push(acc);
    }
    out
}

/// Bug 30 (misc): float equality comparison — `a == b` is unreliable for f64.
pub fn float_eq(a: f64, b: f64) -> bool {
    a == b
}

// Demo function so the crate uses HashSet (otherwise unused-import warning hides
// the trait-consistency bugs above).
pub fn _touch() -> HashSet<u64> { HashSet::new() }
