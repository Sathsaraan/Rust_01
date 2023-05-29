use std::io;

fn add(a : f64,b : f64) -> f64 {
    a + b
}

fn subtract(a : f64,b : f64) -> f64 {
    a - b
}

fn divide(a : f64,b : f64) -> f64 {
    a / b
}

fn multiply(a : f64,b : f64) -> f64 {
    a * b
}

fn power(a: f64, b: f64) -> f64 {
    a.powf(b)
}

fn remainder(a : f64,b : f64) -> f64 {
    a % b
}

fn manipulator(operation: &str, num1: f64, num2: f64) -> String {
    match operation {
        "+" => format!("{} {} {} = {}", num1, operation, num2, add(num1, num2)),
        "-" => format!("{} {} {} = {}", num1, operation, num2, subtract(num1, num2)),
        "*" => format!("{} {} {} = {}", num1, operation, num2, multiply(num1, num2)),
        "/" => format!("{} {} {} = {}", num1, operation, num2, divide(num1, num2)),
        "^" => format!("{} {} {} = {}", num1, operation, num2, power(num1, num2)),
        "%" => format!("{} {} {} = {}", num1, operation, num2, remainder(num1, num2)),
        "#" => String::from("Done!..Terminating..."),
        "?" => String::from("History Records\n"),
        _   => String::from("Wrong operation input!"),
    }
}

fn main() {

    let mut history_records: Vec<String> = Vec::new(); 

    println!("Select operation.");
    println!("1.Add      : + ");
    println!("2.Subtract : - ");
    println!("3.Multiply : * ");
    println!("4.Divide   : / ");
    println!("5.Power    : ^ ");
    println!("6.Remainder: % ");
    println!("7.Terminate: # ");
    println!("8.History  : ? ");
 
    loop {
        let num1: f64;
        let num2: f64;

    // input a
        loop {
            let mut input = String::new();
            println!("\nEnter the first Number: ");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            
            num1 = match input
                .trim()
                .parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter a valid number..");
                        continue;
                    }
                };
            println!("Your first number is: {:?}", num1);
            break;
        };

    //input b
        loop {
            let mut input = String::new();
            println!("\nEnter the second Number: ");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            
            num2 = match input
                .trim()
                .parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter a valid number..");
                        continue;
                    }
                };
            println!("Your second number is: {:?}", num2);
            break;
        };

        let mut operation = String::new();
        println!("\nType the operation you want to perform: ");

        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        let operation = operation.trim();

        if operation == "#" {
            break;
        } else if operation == "?" {
            for record in history_records.clone() {
                println!("{:?}", record);
            }
        } else {
            let results = manipulator(operation, num1, num2);
            history_records.push(results.clone());
            println!("{}", results);
        } 
    } 
}
