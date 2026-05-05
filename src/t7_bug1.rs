// T7 bug 1 (edited): RefCell triple borrow_mut.
use std::cell::RefCell;
pub fn triple_borrow(c: &RefCell<i32>) {
    let _a = c.borrow_mut();
    let _b = c.borrow_mut();
    let _c = c.borrow_mut();
}
