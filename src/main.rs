use std::fs::File;
use std::io;
use std::time::SystemTime;
use io::{stdin, stdout, Write, Read};
use num::bigint::BigUint;

fn new_parameter(ask_string: &str) -> String {
    println!("{}", ask_string);
    let mut parameter = String::new();

    io::stdin()
        .read_line(&mut parameter)
        .expect("Failed to process input.");
    parameter.trim_end().to_string()
}

// Get the final value
fn fibonacci(nth: u128) -> BigUint {
    fib(nth).0
}

// Head recursion
fn fib(nth: u128) -> (BigUint, BigUint) {
    // Break recursion if fib reaches zero
    if nth == 0 {
        return (0u8.into(), 1u8.into());
    }
    // Would number divide evenly by 2?
    let modulo_rem = nth % 2;
    // Subtract nth by modulo rem and divide by 2 to do floor division
    let (a, b): (BigUint, BigUint) = fib((nth - modulo_rem) / 2u128);

    // Algorithm...
    let c = &a * (&b * 2u8 - &a);
    let d = &a * &a + &b * &b;

    if modulo_rem == 1 {
        let summed = c + &d;
        return (d, summed);
    } else {
        return (c, d);
    }
}

fn main() {
    println!("Fibonacci sequence generator!");
    let iterations: u128 = new_parameter("How many iterations would you like?")
        .parse()
        .expect("Failed to parse input as a number");

    let start_time = SystemTime::now();
    let fib_value = fibonacci(iterations);
    let done_duration = SystemTime::now();
    println!("done in {:#?}.", done_duration.duration_since(start_time).unwrap());
    println!("printing to file...");
    let mut file = File::create("./fib_number.txt").unwrap();
    file.write("".to_string().as_bytes()).unwrap();
    file.write(fib_value.to_string().as_bytes()).unwrap();
    println!("file print completed in {:#?}, total time is {:#?}", SystemTime::now().duration_since(done_duration).unwrap(), SystemTime::now().duration_since(start_time).unwrap());
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
