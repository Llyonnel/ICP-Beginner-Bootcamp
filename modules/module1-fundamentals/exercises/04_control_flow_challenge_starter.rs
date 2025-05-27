use std::io;

fn main() {
    // Part 1: FizzBuzz Implementation
    println!("=== FizzBuzz Challenge ===");
    
    // TODO: Implement the FizzBuzz algorithm for numbers 1 to 20
    for i in 1..=20 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
    
    // Part 2: Menu-driven Calculator
    println!("\n=== Calculator ===");
    
    // TODO: Create a variable to control the calculator loop
    let mut running = true;
    
    // TODO: Implement the calculator loop
    while running {
        // TODO: Show the menu options
        println!("Choose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
        
        // TODO: Get the user's choice
        let mut choice = String::new();
        
        // TODO: Read user input
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // TODO: Convert choice to a number (with error handling)
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
        
        // TODO: Exit if the user chose option 5
        if choice == 5 {
            // TODO: Set running to false to exit the loop
            break;
        }
        
        // TODO: Get the two input numbers from the user
        let mut first_number = String::new();
        let mut second_number = String::new();
        println!("Enter the first number: ");
        io::stdin()
            .read_line(&mut first_number)
            .expect("Failed to read line");
        println!("Enter the second number: ");
        io::stdin()
            .read_line(&mut second_number)
            .expect("Failed to read line");

        // Convert input strings to numbers
        let first_number: f64 = match first_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for the first number. Please enter a valid number.");
                continue;
            }
        };
        let second_number: f64 = match second_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input for the second number. Please enter a valid number.");
                continue;
            }
        };
        
        // TODO: Perform the selected operation using match or if statements
        match choice {
           1 => println!("{first_number} + {second_number} = {}",{first_number + second_number}),
           2 => println!("{first_number} - {second_number} = {}",{first_number - second_number}),
           3 => println!("{first_number} * {second_number} = {}",{first_number * second_number}),
           4 => {
               if second_number == 0.0 {
                   println!("Error: Division by zero is not allowed.");
                   continue;
               } else {
                   println!("{first_number} / {second_number} = {}",{first_number / second_number});
               }
           }
           ,
           _ => println!("Invalid option. Please try again."),
        }
        
        // TODO: Ask if the user wants to perform another calculation
        println!("Do you want to perform another calculation? (y/n): ");
        // TODO: Read user's response
        let mut another_calculation: String = String::new();
        io::stdin()
            .read_line(&mut another_calculation)
            .expect("Failed to read line");

        // TODO: Set running to false if the user doesn't want to continue
        if another_calculation.trim().to_lowercase() != "y" {
            running = false;
        }
    }
    
    println!("Thank you for using the calculator!");
}