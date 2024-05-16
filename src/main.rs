// use std::intrinsics::sqrtf64;
use std::io;
use io::{stdin, stdout, Read, Write};
use num::bigint::BigUint;
use num::traits::{One, Zero};

fn new_parameter(ask_string: &str) -> String {
    // clear_terminal_screen();
    println!("{}", ask_string);
    let mut parameter = String::new();
    
    io::stdin()
        .read_line(&mut parameter)
        .expect("Failed to process input.");
    parameter
}

fn main() {
    println!("Fibonacci sequence generator!");
    let iterations: u128 = string_to_int(new_parameter("How many iterations would you like?"));
    // clear_terminal_screen();
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 1..iterations {
        let nth = f0 + f1.clone();
        f0 = f1;
        f1 = nth;
    }
    println!("fib({}) = {}", iterations, f1);
    pause()
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn string_to_int(parameter: String) -> u128 {
    let return_value: u128 = match parameter.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            // clear_terminal_screen();
            println!("Nope, incorrect, input a number!");
            0
        },
    };
    return_value
}