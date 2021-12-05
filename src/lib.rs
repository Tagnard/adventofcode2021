#![allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, BufRead};

pub mod day {
    pub mod one;
    pub mod two;
    pub mod three;
    pub mod four;
    pub mod five;
}

pub struct Input;

impl Input {
    pub fn as_string(input: &str) -> Vec<String> {
        let file = File::open(input).expect("input file not found");
        let reader = BufReader::new(file);
        reader.lines().into_iter().map(|x| x.unwrap()).collect::<Vec<String>>()
    }

    pub fn as_u8(input: &str) -> Vec<u8> {
        let file = File::open(input).expect("input file not found");
        let reader = BufReader::new(file);
        reader.lines().into_iter().map(|x| x.unwrap().parse::<u8>().unwrap()).collect::<Vec<u8>>()
    }

    pub fn as_u32(input: &str) -> Vec<u32> {
        let file = File::open(input).expect("input file not found");
        let reader = BufReader::new(file);
        reader.lines().into_iter().map(|x| x.unwrap().parse::<u32>().unwrap()).collect::<Vec<u32>>()
    }

    pub fn from_binary_string(input: &str) -> Vec<u32> {
        let file = File::open(input).expect("input file not found");
        let reader = BufReader::new(file);
        reader.lines().into_iter().map(|x| u32::from_str_radix(x.unwrap().as_str(), 2).unwrap()).collect::<Vec<u32>>()
    }
}