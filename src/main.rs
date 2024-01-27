// Name: James A. Chase
// File: main.rs
// Date: 23 January 2024
// Description:
//
// This program will be able to detect if a given number sequence is a Fibonacci sequence.

use std::io;

fn fibonacci(n: usize) -> Vec<i32> {
    if n == 1 {
        return vec![0i32];
    }
    if n == 2 {
        return vec![0i32, 1];
    }
    let mut i: usize = 2;
    let mut seq = vec![0i32, 1];
    while i < n {
        seq.push(seq[i-1] + seq[i-2]);
        i += 1;
    }

    seq
}

fn main() {
    // Due to scoping, this is all in a loop to make sure input is valid, but it's really only meant to loop the input if it's invalid.
    loop {
        println!("Enter the number of numbers you'd like to see from the fibonacci sequence [1-47]:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input: usize = match input.trim().parse() {
            Ok(input) => input,
    
            // the underscore here is a catchall value.
            Err(_) => continue,
        };

        if input < 1 || input > 47 {
            println!("Invalid input!");
            continue;
        }
        
        let f = fibonacci(input);
        println!("{f:?}");
        break;
    }
}
