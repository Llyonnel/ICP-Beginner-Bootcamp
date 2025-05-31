// TODO: 1. Define a function that adds two integers and returns the result
fn add(a: i32, b: i32) -> i32 {
    // TODO: Return the sum of a and b
    a + b
}

// TODO: 2. Define a function that calculates the area of a rectangle
fn calculate_rectangle_area(width: f64, height: f64) -> f64 {
    // TODO: Return the area (width × height)
    width * height
}

// TODO: 3. Define a function that checks if a number is prime
fn is_prime(number: u32) -> bool {
    // TODO: Implement the prime number check logic
    //       A number is prime if it's greater than 1 and 
    //       only divisible by 1 and itself
    if number <= 1 {
        return false;
    }
    for i in 2..=((number as f64).sqrt() as u32) {
        if number % i == 0 {
            return false;
        }
    }
    true
}

// TODO: 4. Define a function that converts Fahrenheit to Celsius
// Formula: C = (F - 32) * 5/9
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // TODO: Implement the temperature conversion logic
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    // TODO: Call the addition function with different values and print the results
    let sum1 = add(54, 46);
    let sum2 = add(25, 12);
    println!("Sum of 54 and 46 is: {}", sum1);
    println!("Sum of 25 and 12 is: {}", sum2);
    
    // TODO: Calculate and print the area of rectangles with different dimensions
    let area1 = calculate_rectangle_area(2.5, 4.2);
    let area2 = calculate_rectangle_area(8.3, 12.5);
    println!("Area of rectangle with width 2.5 and height 4.2 is: {:.2} square units", area1);
    println!("Area of rectangle with width 8.3 and height 12.5 is: {:.2} square units", area2);
    
    // TODO: Test your prime number checker with several numbers
    let prime_check1 = is_prime(15);
    let prime_check2 = is_prime(29);
    println!("Is 15 a prime number? {}", prime_check1);
    println!("Is 29 a prime number? {}", prime_check2);


    // TODO: Convert and print some temperatures from Fahrenheit to Celsius
    let celsius1 = fahrenheit_to_celsius(98.6);
    let celsius2 = fahrenheit_to_celsius(32.0);
    println!("98.6°F is equivalent to {:.1}°C", celsius1);
    println!("32.0°F is equivalent to {:.1}°C", celsius2);
    
    // TODO: Print all results with appropriate labels
    // Done!
}