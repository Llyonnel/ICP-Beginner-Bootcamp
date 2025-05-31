fn main() {
    // TODO: 1. Declare an immutable integer variable
    let my_integer:i32 = 42;
    println!("Integer value: {}", my_integer);

    // TODO: 2. Declare a mutable float variable and modify it later
    let mut my_float:f32 = 3.14;
    println!("Original float value: {}", my_float);

    // TODO: Modify the float value
    // my_float = my_float + my_float;
    my_float = my_float * 2.0;
    println!("Modified float value: {}", my_float);
    
    
    // TODO: 3. Declare a boolean variable using type inference
    let is_rust_fun = true;
    println!("Boolean value: {}", is_rust_fun);
    
    // TODO: 4. Declare a character variable with explicit type annotation
    let my_char: char= 'R';
    println!("Character value: {}", my_char);

    // TODO: 5. Perform arithmetic operations with the numeric variables
    let sum = my_float / 2.0 + my_integer as f32;
    let product = my_float * my_integer as f32 / 2.0;
    println!("Addition result: {}", sum);
    println!("Multiplication result: {}", product);
    
    // TODO: 6. Print all variables and calculation results with appropriate labels
    // done
}