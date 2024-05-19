use std::fs::File;
use std::io::{self, Read, Write};
use std::time::SystemTime;
use gmp::mpz::Mpz;

fn new_parameter(ask_string: &str) -> String {
    println!("{}", ask_string);
    let mut parameter = String::new();
    io::stdin().read_line(&mut parameter).expect("Failed to process input.");
    parameter.trim_end().to_string()
}

// Head recursion
fn fib(nth: u128) -> (Mpz, Mpz) {
    // Break recursion if fib reaches zero
    if nth == 0 {
        return (Mpz::zero(), Mpz::one());
    }
    // Would number divide evenly by 2?
    let modulo_rem = nth % 2;
    // Subtract nth by modulo rem and divide by 2 to do floor division
    let (a, b): (Mpz, Mpz) = fib((nth - modulo_rem) / 2);

    // Algorithm...
    let c = &a * (&b * 2u64 - &a);
    let d = &a.pow(2) + &b.pow(2);

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
    let fib_value = fib(iterations);
    let computation_done_time = SystemTime::now();
    println!("Computation done in {:#?}.", computation_done_time.duration_since(start_time).unwrap());

    println!("Printing to file...");
    let mut file = File::create("./fib_number.txt").expect("Failed to create file");
    write!(file, "{}", fib_value.0).expect("Failed to write to file");

    let file_done_time = SystemTime::now();

    println!(
        "File print completed in {:#?}, total time is {:#?}",
        file_done_time.duration_since(computation_done_time).unwrap(),
        file_done_time.duration_since(start_time).unwrap()
    );

    pause();
}

fn pause() {
    let mut stdout = io::stdout();
    stdout.write_all(b"Press Enter to continue...").expect("Failed to write to stdout");
    stdout.flush().expect("Failed to flush stdout");
    io::stdin().read(&mut [0]).expect("Failed to read from stdin");
}
