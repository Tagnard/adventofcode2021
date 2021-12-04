/// --- Day 1: Sonar Sweep ---
/// 
pub fn part_one(input: Vec<u32>) -> u64 {
    
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
fn verify_part_one() {
    assert_eq!(part_one(
        vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ]
    ), 7);
}

pub fn part_two(input: Vec<u32>) -> u32 {

    let mut count: u32 = 0; 
    let mut prev_sum: u32 = 0;
    let mut curr_sum: u32;

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
fn verify_part_two() {
    assert_eq!(part_two(
        vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ]
    ), 5);
}