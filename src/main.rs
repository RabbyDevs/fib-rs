use std::fs::File;
use std::{io, thread};
use std::time::SystemTime;
use io::{stdin, stdout, Write, Read};
use num::bigint::BigUint;
use num::traits::{One, Zero};
use std::sync::mpsc;
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
            // (&a[i][0] * &b[0][j]).clone() + (&a[i][1] * &b[1][j]).clone()
            let (tx, rx) = mpsc::channel::<BigUint>();
            let (tp, rp) = mpsc::channel::<BigUint>();
            let matrix_value1 = a[i][0].clone();
            let matrix_value2 = b[0][j].clone();
            let matrix_value3 = a[i][1].clone();
            let matrix_value4 = b[1][j].clone();
            thread::spawn(move || {
                tx.send((matrix_value1 * matrix_value2).clone()).unwrap()
            });
            thread::spawn(move || {
                tp.send((matrix_value3 * matrix_value4).clone()).unwrap()
            });
            let matrix1 = rx.recv().unwrap();
            let matrix2 = rp.recv().unwrap();
            matrix1 + matrix2
        })
    });
    c
}

fn matrix_power(mut base: [[BigUint; 2]; 2], mut exp: u128) -> [[BigUint; 2]; 2] {
    let mut result: [[BigUint; 2]; 2] = [
        [BigUint::zero(), BigUint::one()],
        [BigUint::one(), BigUint::zero()],
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
