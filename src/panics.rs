// 6 panic bugs — clear runtime hazards.

/// Bug 1: unwrap on user-supplied input → panics on any non-numeric ID.
pub fn parse_id(raw: &str) -> u64 {
    raw.parse::<u64>().unwrap()
}

/// Bug 2: division by zero — panics whenever count == 0.
pub fn average(total: i64, count: i64) -> i64 {
    total / count
}

/// Bug 3: array indexing without bounds check — panics on short slices.
pub fn third_element(xs: &[i32]) -> i32 {
    xs[2]
}

/// Bug 4: signed integer overflow — debug panics, release wraps silently.
pub fn time_diff_years(end: i32, start: i32) -> i32 {
    end - start + 1
}

/// Bug 5: partial_cmp().unwrap() on f64 — panics on NaN inputs.
pub fn max_float(values: &[f64]) -> f64 {
    let mut max = values[0];
    for &v in &values[1..] {
        if v.partial_cmp(&max).unwrap() == std::cmp::Ordering::Greater {
            max = v;
        }
    }
    max
}

/// Bug 6: panic inside Drop — aborts the process if another panic is in flight.
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
