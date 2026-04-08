use std::collections::VecDeque;

pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push_back(item);
    }

    // BUG: panics on empty queue. Should return Option<T>.
    pub fn pop(&mut self) -> T {
        self.items.pop_front().unwrap()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
