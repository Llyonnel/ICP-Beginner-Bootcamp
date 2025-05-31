mod product;
mod user;
mod order;
mod inventory;

use product::Product;
use user::User;
use order::{Order, OrderStatus};
use inventory::Inventory;

fn main() {
    // Create some products
    let product1 = Product::new(1, "Laptop".to_string(), 999.99, "High-performance laptop".to_string());
    let product2 = Product::new(2, "Smartphone".to_string(), 499.99, "Latest smartphone".to_string());
    let product3 = Product::new(3, "Television".to_string(), 299.99, "A old school television".to_string());

    // Display product1 (Laptop)
    println!("Details of Laptop product:");
    product1.display();

    println!("\nDetails of Smartphone product:");
    product2.display();

    println!("\nDetails of Television product:");
    product3.display();

    // Add products to inventory
    let mut inventory = Inventory::new();
    inventory.add_product(1, 10);
    inventory.add_product(2, 20);
    inventory.add_product(3, 15);

    // Remove products from inventory
    inventory.remove_product(3, 5);

    // Check stock in inventory
    println!("\nThere are {} Laptop in the inventory", inventory.check_stock(1));
    println!("\nThere are {} Television in the inventory", inventory.check_stock(3));

    // Create a user
    let user = User::new(1, "John Doe".to_string(), "john.doe@example.com".to_string(), "123 Main St".to_string());

    // Display an user (John Doe)
    println!("\nDetails of John Doe");
    user.display();

    // Create an order for the user with some products
    let mut order = Order::new(1, user, vec![product1, product2]);
    order.update_status(OrderStatus::Shipped);

    // Display order details
    println!("\nDetails of order:");
    order.display();
}