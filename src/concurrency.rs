//! Planted bugs: concurrency category.
//!   1. Data race — shared &mut via raw pointer
//!   2. Deadlock — lock order inversion across two Mutexes
//!   3. Arc::clone without Mutex on a counter (use AtomicUsize instead)
//!   4. join().unwrap() — panic propagation swallowed silently

use std::sync::{Arc, Mutex};
use std::thread;

pub fn racing_counter() -> u64 {
    let counter = 0u64;
    let counter_ptr: *mut u64 = &counter as *const u64 as *mut u64;
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let p = counter_ptr as usize;
            thread::spawn(move || {
                let p = p as *mut u64;
                for _ in 0..10_000 {
                    // SAFETY: none — classic data race
                    unsafe {
                        *p += 1;
                    }
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    counter
}

pub fn deadlock_two_locks(a: &Arc<Mutex<i32>>, b: &Arc<Mutex<i32>>) {
    let a1 = Arc::clone(a);
    let b1 = Arc::clone(b);
    let a2 = Arc::clone(a);
    let b2 = Arc::clone(b);

    let h1 = thread::spawn(move || {
        let _ga = a1.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _gb = b1.lock().unwrap();
    });
    let h2 = thread::spawn(move || {
        let _gb = b2.lock().unwrap();
        thread::sleep(std::time::Duration::from_millis(10));
        let _ga = a2.lock().unwrap();
    });
    h1.join().unwrap();
    h2.join().unwrap();
}

pub struct Counter {
    pub value: u64,
}

pub fn increment_shared(c: Arc<Counter>) {
    let counter_addr = Arc::as_ptr(&c) as *mut Counter;
    unsafe {
        (*counter_addr).value += 1;
    }
}
