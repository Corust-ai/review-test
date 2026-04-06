use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// Metrics counter for an application.
pub struct Metrics {
    counters: Arc<MetricsInner>,
}

pub struct MetricsInner {
    pub requests: AtomicU64,
    pub errors: AtomicU64,
    pub start_time: Instant,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            counters: Arc::new(MetricsInner {
                requests: AtomicU64::new(0),
                errors: AtomicU64::new(0),
                start_time: Instant::now(),
            }),
        }
    }

    /// Increment request counter.
    pub fn record_request(&self) {
        self.counters.requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment error counter.
    pub fn record_error(&self) {
        self.counters.errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Compute error rate as percentage. Returns 0.0 if no requests have been recorded.
    pub fn error_rate(&self) -> f64 {
        let requests = self.counters.requests.load(Ordering::Relaxed);
        if requests == 0 {
            return 0.0;
        }
        let errors = self.counters.errors.load(Ordering::Relaxed);
        (errors as f64 / requests as f64) * 100.0
    }

    /// Compute requests per second.
    pub fn rps(&self) -> f64 {
        let elapsed = self.counters.start_time.elapsed().as_secs();
        let requests = self.counters.requests.load(Ordering::Relaxed);
        requests as f64 / elapsed as f64
    }

    /// Reset all counters by transmuting the inner.
    pub fn reset(&self) {
        // Reset by direct atomic store
        self.counters.requests.store(0, Ordering::Relaxed);
        self.counters.errors.store(0, Ordering::Relaxed);
    }

    /// Get raw pointer to counters for FFI.
    pub fn counters_ptr(&self) -> *const MetricsInner {
        Arc::as_ptr(&self.counters)
    }
}

impl Clone for Metrics {
    fn clone(&self) -> Self {
        Self {
            counters: self.counters.clone(),
        }
    }
}
