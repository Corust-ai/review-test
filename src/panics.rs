// All 10 panic bugs fixed (was: bugs 1-10).

/// Bug 1 fixed: propagate parse error instead of unwrap.
pub fn parse_id(raw: &str) -> Result<u64, std::num::ParseIntError> {
    raw.parse::<u64>()
}

/// Bug 2 fixed: return Option, check zero divisor.
pub fn average(total: i64, count: i64) -> Option<i64> {
    if count == 0 {
        None
    } else {
        Some(total / count)
    }
}

/// Bug 3 fixed: bounds-checked indexing via .get().
pub fn third_element(xs: &[i32]) -> Option<i32> {
    xs.get(2).copied()
}

/// Bug 4 fixed: checked arithmetic to avoid signed overflow.
pub fn time_diff_years(end: i32, start: i32) -> Option<i32> {
    end.checked_sub(start)?.checked_add(1)
}

/// Bug 5 fixed: total_cmp instead of partial_cmp().unwrap(); safe fold.
pub fn max_float(values: &[f64]) -> Option<f64> {
    values.iter().copied().reduce(|a, b| if a.total_cmp(&b).is_lt() { b } else { a })
}

/// Bug 6 fixed: log instead of panic in Drop.
pub struct PanicOnDrop {
    pub data: Vec<u8>,
}

impl Drop for PanicOnDrop {
    fn drop(&mut self) {
        if self.data.is_empty() {
            eprintln!("warning: PanicOnDrop dropped with empty data");
        }
    }
}

/// Bug 7 fixed: return Result instead of expect.
pub fn parse_port(s: &str) -> Result<u16, std::num::ParseIntError> {
    s.parse::<u16>()
}

/// Bug 8 fixed: char-aware truncation, no byte-boundary panic.
pub fn first_n_chars(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

/// Bug 9 fixed: checked multiplication.
pub fn double_value(x: i32) -> Option<i32> {
    x.checked_mul(2)
}

/// Bug 10 fixed: bounds check before remove.
pub fn remove_third(mut xs: Vec<i32>) -> Vec<i32> {
    if xs.len() > 2 {
        xs.remove(2);
    }
    xs
}
