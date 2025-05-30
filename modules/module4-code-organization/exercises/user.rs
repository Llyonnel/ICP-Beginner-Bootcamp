pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub address: String,
}

impl User {
    pub fn new(id: u32, name: String, email: String, address: String) -> Self {
        User {
            id,
            name,
            email,
            address,
        }
    }

    pub fn display(&self) {
        println!("User ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Email: {}", self.email);
        println!("Address: {}", self.address);
    }
}