use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub quantity: u32,
    pub price_cents: u64,
}

pub struct Inventory {
    items: HashMap<u64, Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: Item) {
        self.items.insert(item.id, item);
    }

    pub fn get(&self, id: u64) -> Option<&Item> {
        self.items.get(&id)
    }

    // BUG 2: subtract overflow when quantity is 0
    pub fn remove_one(&mut self, id: u64) {
        if let Some(item) = self.items.get_mut(&id) {
            item.quantity = item.quantity.saturating_sub(1);
        }
    }

    // BUG 3: integer overflow on price * quantity for large values
    pub fn total_value(&self) -> Option<u64> {
        self.items.values().try_fold(0u64, |acc, i| {
            i.price_cents.checked_mul(i.quantity as u64)?.checked_add(acc)
        })
    }

    // BUG 4: divide by zero when inventory is empty
    pub fn average_price(&self) -> Option<u64> {
        let len = self.items.len() as u64;
        if len == 0 {
            return None;
        }
        let total: u64 = self.items.values().map(|i| i.price_cents).sum();
        Some(total / len)
    }

    // INTENTIONAL: explicit panic for debugging — we want to keep this
    pub fn debug_dump(&self) {
        if self.items.is_empty() {
            panic!("debug_dump called on empty inventory");
        }
        for item in self.items.values() {
            println!("{:?}", item);
        }
    }
}
