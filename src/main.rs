// Name: James A. Chase
// File: main.rs
// Date: 23 January 2024
// Description:
//
// This program will be able to detect if a given number sequence is a Fibonacci sequence.
// For the purpose of this project, a sequence is determined to be a Fibonacci sequence if after the first two numbers in the sequence, every
// following number is the sum of the two numbers preceeding it in the sequence.

// include proper imports for functions ------------------

// These allow us to write the results
use std::io::Write;
use std::fs::File;

// These allow us to remove the results.txt file at the beginning of the program
use std::path::Path;
use std::fs::remove_file;

// This allows us to read in the sequences.txt file
use std::fs::read_to_string;

// ---------------------------------

// This function very simply reads in the file and compiles it into a vector of Strings
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines() // makes an iterator of String slices
        .map(String::from) // converts each slice into a String
        .collect() // makes vector
}

// This function is used on each sequence to determine if it is a Fibonacci sequence or not!
fn detect_fibonacci(seq: Vec<i32>) -> bool {
    // start from the end of the sequence
    let mut i = seq.len() - 1;

    // if the sequence is 2 or less in length, it is not a valid Fibonacci sequence
    if i < 2 { return false; }

    // iterate while we're still not at the first two numbers
    while i > 1 {

        // if at any point the sequence becomes invalid, exit returning false
        if seq[i] != seq[i-1] + seq[i-2] {
            return false;
        }

        i -= 1;
    }

    // if loop breaks, sequence must be valid
    true
}

// This function handles each sequence provided in sequences.txt
fn compute_sequences(lines: Vec<String>) {
    for seq in lines {
        // handle if blank line
        if seq.len() == 0 { continue; }

        // handle if a character in sequence is not a digit
        if !seq.chars().filter(|c| !c.is_whitespace()).all(char::is_numeric) { continue; }

        // new vector to hold the converted numbers from the provided sequence String
        let mut new_vec: Vec<i32> = Vec::new();
        
        // here we split the string along whitespace since each sequence is space-separated within itself
        for split in seq.trim().split_whitespace() {
            // parse the value and push into new vector
            new_vec.push(split.parse().expect("Error parsing number"));
        }

        // take new vector into this function to write the resulting true or false value
        // underscore used to hold Result that we aren't doing anything with (clears compiler warning)
        let _ = write_results(new_vec);
    }
}

// This function calls the detect_fibonacci function to obtain the result and then writes that result to results.txt
fn write_results(seq: Vec<i32>) -> std::io::Result<()> {
    // call detect_fibonacci
    let result = detect_fibonacci(seq);

    // create file if it doesn't exist, then use append mode so each sequence result can be written
    let mut file = File::options().create(true).append(true).open("results.txt")?;

    // write result
    writeln!(&mut file, "{result}")?;
    Ok(())
}

// This function checks to see if results.txt exists or not, and deletes it if it does.
// This is due to the fact that we open the file in append mode, so if you change sequences.txt, results.txt would become very unclear
fn clean_results() -> std::io::Result<()> {
    if Path::new("results.txt").exists() {
        remove_file("results.txt")?;
    }
    Ok(())
}

fn main() {
    // removes the results file on run so that the fresh file can be created and used.
    // underscore again used to hold Result that we aren't doing anything with (clears compiler warning)
    let _ = clean_results();

    // Read the lines from the sequences.txt file and store each as a String inside of a vector
    let file_lines = read_lines("sequences.txt");

    // compute whether or not each sequence is a valid fibonacci sequence
    compute_sequences(file_lines);
}
