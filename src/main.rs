// use std::intrinsics::sqrtf64;
use std::io;
use std::str::FromStr;
use io::{stdin, stdout, Read, Write};
use num::bigint::BigUint;
use num::traits::{One, Zero};
use num::{iter, BigInt, FromPrimitive};

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
    let iterations: BigUint = BigUint::from_str(new_parameter("How many iterations would you like?").as_str().trim_end()).unwrap();
    // clear_terminal_screen();
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in num_iter::range_inclusive(BigUint::from_u16(1).unwrap(), iterations.clone()) {
        let nth = f0 + &f1;
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