#![allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn get_lines(input: &str) -> Vec<String> {
    let file = File::open(input).expect("input file not found");
    let reader = BufReader::new(file);

    let mut output: Vec<String> = Vec::new();

    for line in reader.lines() {
        output.push(line.unwrap())
    }

    output
}

pub fn day_1_part_1(input: Vec<String>) -> i64 {
    let input: Vec<i64> = input.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    
    let mut inc = 0;
    let mut dec = 0;

    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            inc = inc + 1;
        } else {
            dec = dec + 1;
        };
    }

    inc
}

#[test]
fn verify_day_1_part_1() {
    assert_eq!(day_1_part_1(
        vec![
            "199".to_owned(), 
            "200".to_owned(), 
            "208".to_owned(), 
            "210".to_owned(), 
            "200".to_owned(), 
            "207".to_owned(),
            "240".to_owned(),
            "269".to_owned(),
            "260".to_owned(),
            "263".to_owned(),
        ]
    ), 7);
}

pub fn day_1_part_2(input: Vec<String>) -> i64 {
    let input: Vec<i64> = input.iter().map(|x| x.parse::<i64>().unwrap()).collect();

    let mut count: i64 = 0; 
    let mut prev_sum: i64 = 0;
    let mut curr_sum: i64;

    for i in 1..input.len() -1 {
        let a = input[i -1];
        let b = input[i];
        let c = input[i+1];

        curr_sum = a + b + c;

        if prev_sum > 0 && curr_sum > prev_sum {
            count = count + 1
        };

        prev_sum = curr_sum
    }

    count
}

#[test]
fn verify_day_1_part_2() {
    assert_eq!(day_1_part_2(
        vec![
            "199".to_owned(), 
            "200".to_owned(), 
            "208".to_owned(), 
            "210".to_owned(), 
            "200".to_owned(), 
            "207".to_owned(),
            "240".to_owned(),
            "269".to_owned(),
            "260".to_owned(),
            "263".to_owned(),
        ]
    ), 5);
}

pub fn day_2_part_1(input: Vec<String>) -> i64 {
    let mut position: (i64, i64) = (0, 0);

    for line in input {
        let parts: Vec<&str> = line.split(" ").collect();
        let direction = parts[0];
        let amount = parts[1].parse::<i64>().unwrap();

        match direction {
            "up" => position.1 = position.1 - amount,
            "down" => position.1 = position.1 + amount,
            "forward" => position.0 = position.0 + amount,
            _  => (),
        };
    }

    position.0 * position.1
}

#[test]
fn verify_day_2_part_1() {
    assert_eq!(day_2_part_1(vec![
        "forward 5".to_owned(), 
        "down 5".to_owned(), 
        "forward 8".to_owned(), 
        "up 3".to_owned(), 
        "down 8".to_owned(), 
        "forward 2".to_owned()]), 150);
}

pub fn day_2_part_2(input: Vec<String>) -> i64 {
    let mut horizontal_position: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;

    for line in input {
        let parts: Vec<&str> = line.split(" ").collect();
        let direction = parts[0];
        let amount = parts[1].parse::<i64>().unwrap();

        match direction {
            "up" => aim = aim - amount,
            "down" => aim = aim + amount,
            "forward" => {
                horizontal_position = horizontal_position + amount;
                depth = depth + amount * aim;
            },
            _  => (),
        };
    }

    horizontal_position * depth
}

#[test]
fn verify_day_2_part_2() {
    assert_eq!(day_2_part_2(vec![
        "forward 5".to_owned(), 
        "down 5".to_owned(), 
        "forward 8".to_owned(), 
        "up 3".to_owned(), 
        "down 8".to_owned(), 
        "forward 2".to_owned()]), 900);
}