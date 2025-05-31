use std::collections::HashMap;

pub struct Inventory {
    stock: HashMap<u32, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            stock: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product_id: u32, quantity: u32) {
        *self.stock.entry(product_id).or_insert(0) += quantity;
    }

    pub fn remove_product(&mut self, product_id: u32, quantity: u32) -> bool {
        if let Some(stock_quantity) = self.stock.get_mut(&product_id) {
            if *stock_quantity >= quantity {
                *stock_quantity -= quantity;
                return true;
            }
        }
        false
    }

    pub fn check_stock(&self, product_id: u32) -> u32 {
        *self.stock.get(&product_id).unwrap_or(&0)
    }
}