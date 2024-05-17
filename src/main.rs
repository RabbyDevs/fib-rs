use std::io;
use std::time::SystemTime;
use io::{stdin, stdout, Write, Read};
use num::bigint::BigUint;
use num::traits::{One, Zero};
use std::array::from_fn;

fn new_parameter(ask_string: &str) -> String {
    println!("{}", ask_string);
    let mut parameter = String::new();

    io::stdin()
        .read_line(&mut parameter)
        .expect("Failed to process input.");
    parameter.trim_end().to_string()
}

fn matrix_multiply(a: &[[BigUint; 2]; 2], b: &[[BigUint; 2]; 2]) -> [[BigUint; 2]; 2] {
    let c = from_fn(|i| {
        from_fn(|j| {
            (&a[i][0] * &b[0][j]).clone() + (&a[i][1] * &b[1][j]).clone()
        })
    });
    c
}

fn matrix_power(mut base: [[BigUint; 2]; 2], mut exp: u128) -> [[BigUint; 2]; 2] {
    let mut result = [
        [BigUint::one(), BigUint::zero()],
        [BigUint::zero(), BigUint::one()],
    ];
    while exp > 0 {
        if exp % 2 == 1 {
            result = matrix_multiply(&result, &base);
        }
        base = matrix_multiply(&base, &base);
        exp /= 2;
    }
    result
}

fn fibonacci(n: u128) -> BigUint {
    if n == 0 {
        return BigUint::zero();
    }
    let fib_matrix = [
        [BigUint::one(), BigUint::one()],
        [BigUint::one(), BigUint::zero()],
    ];
    let result_matrix = matrix_power(fib_matrix, n);
    result_matrix[0][0].clone()
}

fn main() {
    println!("Fibonacci sequence generator!");
    let iterations: u128 = new_parameter("How many iterations would you like?")
        .parse()
        .expect("Failed to parse input as a number");

    let start_time = SystemTime::now();
    let fib_value = fibonacci(iterations);
    println!("fib({}) = {}", iterations, fib_value);
    println!("done in {:#?}.", SystemTime::now().duration_since(start_time).unwrap());
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
