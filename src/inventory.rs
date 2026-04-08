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

    // BUG 1: panics if id not found — should return Option<&Item>
    pub fn get(&self, id: u64) -> &Item {
        self.items.get(&id).unwrap()
    }

    // BUG 2: subtract overflow when quantity is 0
    pub fn remove_one(&mut self, id: u64) {
        if let Some(item) = self.items.get_mut(&id) {
            item.quantity -= 1;
        }
    }

    // BUG 3: integer overflow on price * quantity for large values
    pub fn total_value(&self) -> u64 {
        self.items.values().map(|i| i.price_cents * i.quantity as u64).sum()
    }

    // BUG 4: divide by zero when inventory is empty
    pub fn average_price(&self) -> u64 {
        let total: u64 = self.items.values().map(|i| i.price_cents).sum();
        total / self.items.len() as u64
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
