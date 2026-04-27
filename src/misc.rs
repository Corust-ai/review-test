// 10 misc bugs (Bugs 41-43 fixed: removed).

#![allow(unused_imports)]
use std::collections::HashSet;

/// Bug 44: lossy cast i64 → u32.
pub fn id_as_u32(id: i64) -> u32 {
    id as u32
}

/// Bug 45: clone of large struct in hot loop.
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

/// Bug 46: float equality — unreliable for f64.
pub fn float_eq(a: f64, b: f64) -> bool {
    a == b
}

/// Bug 47: PartialOrd / Ord disagreement.
#[derive(PartialEq, Eq, PartialOrd)]
pub struct Priority(pub u32);

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

/// Bug 48: boolean trap.
pub fn set_logging(enable: bool, verbose: bool, color: bool) {
    if enable && verbose && color {
        eprintln!("logging configured");
    }
}

/// Bug 49: lossy widening cast i64 → f32 in money calc.
pub fn cents_to_display(cents: i64) -> f32 {
    cents as f32 / 100.0
}

/// Bug 50: shift overflow.
pub fn shift_left(x: i32, by: u32) -> i32 {
    x << by
}

pub fn _touch() -> HashSet<u64> { HashSet::new() }
