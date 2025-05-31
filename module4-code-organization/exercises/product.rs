pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub description: String,
}

impl Product {
    pub fn new(id: u32, name: String, price: f64, description: String) -> Self {
        Product {
            id,
            name,
            price,
            description,
        }
    }

    pub fn display(&self) {
        println!("Product ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Price: {}", self.price);
        println!("Description: {}", self.description);
    }
}