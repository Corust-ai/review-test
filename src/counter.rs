pub struct Counter {
    value: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    // BUG: subtract overflow when value is 0
    pub fn decrement(&mut self) {
        self.value = self.value.saturating_sub(1);
    }

    pub fn increment(&mut self) {
        self.value = self.value.saturating_add(1);
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
