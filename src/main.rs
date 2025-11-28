use std::io;

fn main() {
    println!("Enter first Number: ");
    let a = READ_NUMBER();

    println!("Enter Operator: ");
    let op = READ_OPERATOR();

    println!("Enter Second Number: ");
    let b = READ_NUMBER();

    let result = CALCULATE(a, b, op);

    match result {
        Ok(e) => println!("Result: {e}\n"),
        Err(r) => println!("Error: {r}\n"),
    }
}

fn READ_NUMBER() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if let Ok(num) = input.trim().parse::<f64>() {
            return num;
        }
        println!("Please enter a valid number:");
    }
}

fn READ_OPERATOR() -> char {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if let Some(e) = input.trim().chars().next() {
            if "+-*/".contains(e) {
                return e;
            }
        }
        println!("Please enter a valid operator (+, -, *, /):");
    }
}

fn CALCULATE(a: f64, b: f64, op: char) -> Result<f64, &'static str> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err("NO DIVIDE BY ZERO")
            } else {
                Ok(a / b)
            }
        }
        _ => Err("Unknown Operator"),
    }
}