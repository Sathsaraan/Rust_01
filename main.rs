use std::io::{self, empty};

fn add(a : f64,b : f64) -> f64 {
    a + b
}

fn subtract(a : f64,b : f64) -> f64 {
    a - b
}
 
//what if b == 0

fn divide(a : f64,b : f64) -> f64 {
    a / b
}

fn multiply(a : f64,b : f64) -> f64 {
    a * b
}

fn remainder(a : f64,b : f64) -> f64 {
    a % b
}

fn manipulator(operation_val: String, num1: f64, num2: f64) -> (f64, String) {
    if operation_val == "+" {
        (add(num1, num2), String::from("add"))
    } 
    else if operation_val == "-" {
        (subtract(num1, num2), String::from("subtract"))
    } 
    else if operation_val == "/" {
        if num2 == 0.0 {
            (0.0, String::from("Cannot divide by ZERO!"))
        } else {
            (divide(num1, num2), String::from("divide"))
        }
    } 
    else if operation_val == "*" {
            (multiply(num1, num2), String::from("multiply"))
    } 
    else if operation_val == "%" {
        if num2 == 0.0 {
            (0.0, String::from("Cannot divide by ZERO!"))
        } else {
            (remainder(num1, num2), String::from("remainder"))
        }
    } 
    else {
        (0.0, String::from("Wrong Operation Choice!"))
    }
}

fn main() {
    println!("Select operation.");
    println!("1.Add      : + ");
    println!("2.Subtract : - ");
    println!("3.Multiply : * ");
    println!("4.Divide   : / ");
    println!("5.Remainder: % ");

//----------------------------------------------------------------------------------------------------------------
// input a
    let mut a = String::new();
    println!("Enter the first Number: ");
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
    
    let num1: f64 = a
        .trim()
        .parse()
        .expect("Enter a valid number");

//input b
    let mut b = String::new();
    println!("Enter the second Number: ");
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line");
    
    let num2: f64 = b
        .trim()
        .parse()
        .expect("Enter a valid number");
//---------------------------------------------------------------------------------------------------------------

    let mut operation = String::new();
    println!("Type the operation you want to perform: ");

    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    let operation_val: String = operation
        .trim()
        .parse()
        .expect("Parse incorrectly");

    
 let finale_results = manipulator(operation_val.clone(), num1, num2);
 println!("{} {} {} = {}", num1, operation_val, num2, finale_results.0);
} 

