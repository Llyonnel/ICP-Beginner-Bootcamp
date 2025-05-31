use std::io;
use chrono::prelude::*;

fn main() {
    // TODO: 1. Prompt the user for their name
    println!("What's your name?");
    
    // TODO: 2. Read the user's input
    let mut name: String = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    name = String:: from(name.trim());

    // other solution, but change type
    // let name: &str = name.trim(); 
    
    // TODO: 3. Print a personalized greeting
    println!("Hello dear {name}");

    // BONUS: Print the current date
    let time: DateTime<Utc> = Utc::now();
    println!("{time}");
    // Hint: You can use the chrono crate for this
}