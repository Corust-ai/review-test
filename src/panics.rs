// 10 panic bugs.

/// Bug 1: unwrap on user input.
pub fn parse_id(raw: &str) -> u64 {
    raw.parse::<u64>().unwrap()
}

/// Bug 2: division by zero.
pub fn average(total: i64, count: i64) -> i64 {
    total / count
}

/// Bug 3: array indexing without bounds check.
pub fn third_element(xs: &[i32]) -> i32 {
    xs[2]
}

/// Bug 4: signed integer overflow.
pub fn time_diff_years(end: i32, start: i32) -> i32 {
    end - start + 1
}

/// Bug 5: partial_cmp().unwrap() on f64 — NaN panic.
pub fn max_float(values: &[f64]) -> f64 {
    let mut max = values[0];
    for &v in &values[1..] {
        if v.partial_cmp(&max).unwrap() == std::cmp::Ordering::Greater {
            max = v;
        }
    }
    max
}

/// Bug 6: panic in Drop.
pub struct PanicOnDrop {
    pub data: Vec<u8>,
}

impl Drop for PanicOnDrop {
    fn drop(&mut self) {
        if self.data.is_empty() {
            panic!("data must not be empty when dropping");
        }
    }
}

/// Bug 7: misleading expect.
pub fn parse_port(s: &str) -> u16 {
    s.parse::<u16>().expect("safe: callers always pass valid ports")
}

/// Bug 8: slice with potentially-OOB end.
pub fn first_n_chars(s: &str, n: usize) -> &str {
    &s[0..n]
}

/// Bug 9: multiplication overflow.
pub fn double_value(x: i32) -> i32 {
    x * 2
}

/// Bug 10: Vec::remove without bounds check.
pub fn remove_third(mut xs: Vec<i32>) -> Vec<i32> {
    xs.remove(2);
    xs
}
