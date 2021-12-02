#![allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, BufRead};

pub mod day {
    pub mod one;
    pub mod two;
}

pub fn get_lines(input: &str) -> Vec<String> {
    let file = File::open(input).expect("input file not found");
    let reader = BufReader::new(file);

    let mut output: Vec<String> = Vec::new();

    for line in reader.lines() {
        output.push(line.unwrap())
    }

    output
}