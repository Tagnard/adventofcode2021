/// --- Day 1: Sonar Sweep ---
/// 
pub fn part_one(input: Vec<String>) -> i64 {
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
fn verify_part_one() {
    assert_eq!(part_one(
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

pub fn part_two(input: Vec<String>) -> i64 {
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
fn verify_part_two() {
    assert_eq!(part_two(
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