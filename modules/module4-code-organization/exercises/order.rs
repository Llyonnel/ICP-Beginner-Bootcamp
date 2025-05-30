use crate::product::Product;
use crate::user::User;

#[derive(Debug)]
pub enum OrderStatus {
    Pending,
    Shipped,
    // Delivered,
}

pub struct Order {
    pub id: u32,
    pub user: User,
    pub products: Vec<Product>,
    pub status: OrderStatus,
}

impl Order {
    pub fn new(id: u32, user: User, products: Vec<Product>) -> Self {
        Order {
            id,
            user,
            products,
            status: OrderStatus::Pending,
        }
    }

    pub fn update_status(&mut self, status: OrderStatus) {
        self.status = status;
    }

    pub fn display(&self) {
        println!("Order ID: {}", self.id);
        println!("User: {}", self.user.name);
        println!("Products:");
        for product in &self.products {
            println!("- {}", product.name);
        }
        println!("Status: {:?}", self.status);
    }
}