pub struct Stats {
    values: Vec<f64>,
}

impl Stats {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn add(&mut self, v: f64) {
        self.values.push(v);
    }

    pub fn mean(&self) -> f64 {
        let mut sum = 0.0;
        for v in &self.values {
            sum += v;
        }
        sum / self.values.len() as f64
    }

    pub fn max(&self) -> f64 {
        let mut m = self.values[0];
        for v in &self.values {
            if *v > m {
                m = *v;
            }
        }
        m
    }
}
