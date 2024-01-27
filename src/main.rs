// Name: James A. Chase
// File: main.rs
// Date: 23 January 2024
// Description:
//
// This program will be able to detect if a given number sequence is a Fibonacci sequence.

use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::fs::remove_file;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn detect_fibonacci(seq: &Vec<i32>) -> bool {
    let mut i = seq.len() - 1;
    while i > 2 {
        if seq[i] != seq[i-1] + seq[i-2] {
            return false;
        }

        i -= 1;
    }

    true
}

fn compute_sequences(lines: Vec<String>) {
    for seq in lines {
        let mut new_vec: Vec<i32> = Vec::new();
        for split in seq.trim().split_whitespace() {
            new_vec.push(split.parse().expect("Error parsing number"));
        }
        let _ = write_results(&new_vec);
    }
}

fn write_results(seq: &Vec<i32>) -> std::io::Result<()> {
    let result = detect_fibonacci(&seq);
    let mut file = File::options().create(true).append(true).open("results.txt")?;
    writeln!(&mut file, "{result}")?;
    Ok(())
}

fn clean_results() -> std::io::Result<()> {
    if Path::new("results.txt").exists() {
        remove_file("results.txt")?;
    }
    Ok(())
}

fn main() {
    let _ = clean_results();

    let file_lines = read_lines("sequences.txt");

    compute_sequences(file_lines);
}
