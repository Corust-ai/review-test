use std::cmp::Ordering;

// Bug 41 — signed integer overflow (silent wrap in --release)
pub fn factorial(n: i64) -> i64 {
    let mut r: i64 = 1;
    for i in 1..=n {
        r = r * i;
    }
    r
}

// Bug 42 — division by zero on user input
pub fn average(values: &[i64], n: i64) -> i64 {
    let sum: i64 = values.iter().sum();
    sum / n
}

// Bug 43 — float equality comparison (NaN unsafe)
pub fn prices_match(a: f64, b: f64) -> bool {
    a == b
}

// Bug 44 — lossy widening cast hiding signed overflow
pub fn add_u32(a: u32, b: u32) -> u64 {
    (a + b) as u64
}

// Bug 45 — f64 for money (precision loss)
pub struct Order {
    pub price: f64,
    pub quantity: f64,
}

impl Order {
    pub fn total(&self) -> f64 {
        self.price * self.quantity
    }
}

// Bug 46 — shift overflow (1 << n for n >= 64 is UB on u64)
pub fn bit_at(value: u64, n: u32) -> bool {
    (value & (1u64 << n)) != 0
}

// Bug 47 — unsigned subtract underflow (a < b panics or wraps)
pub fn range_size(start: usize, end: usize) -> usize {
    end - start
}

// Bug 48 — i32 → usize cast (negative becomes huge index, panic on bounds)
pub fn fetch_at(arr: &[u8], idx: i32) -> u8 {
    arr[idx as usize]
}

// Bug 49 — partial_cmp().unwrap() on f64 (NaN panic)
pub fn sort_by_price(items: &mut Vec<f64>) {
    items.sort_by(|a, b| a.partial_cmp(b).unwrap());
}

// Bug 50 — mem::transmute for byte split (alignment + endianness footgun)
pub fn u32_to_bytes(val: u32) -> [u8; 4] {
    unsafe { std::mem::transmute::<u32, [u8; 4]>(val) }
}
