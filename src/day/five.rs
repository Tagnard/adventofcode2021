use std::collections::HashMap;
use regex::Regex;


pub fn part_one(input: Vec<String>) -> usize {    
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    for line in input {
        let cap = re.captures(&line).unwrap();
        let x1 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y1 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let x2 = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let y2 = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();

        if x1 == x2 {
            for y in y1.min(y2) ..= y2.max(y1) {
                *grid.entry((x1, y)).or_default() += 1;
            }
        }
        if y1 == y2 {
            for x in x1.min(x2) ..= x2.max(x1) {
                *grid.entry((x, y1)).or_default() += 1;
            }
        }
    }

    grid.values().filter(|v| **v >= 2).count()
}

#[test]
fn verify_part_one() {
    assert_eq!(part_one(
        vec![
            "0,9 -> 5,9".to_owned(),
            "8,0 -> 0,8".to_owned(),
            "9,4 -> 3,4".to_owned(),
            "2,2 -> 2,1".to_owned(),
            "7,0 -> 7,4".to_owned(),
            "6,4 -> 2,0".to_owned(),
            "0,9 -> 2,9".to_owned(),
            "3,4 -> 1,4".to_owned(),
            "0,0 -> 8,8".to_owned(),
            "5,5 -> 8,2".to_owned(),
        ]
    ), 5);
}

pub fn part_two(input: Vec<String>) -> usize {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();

    for line in input {
        let cap = re.captures(&line).unwrap();
        let x1 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y1 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let x2 = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let y2 = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();

        if x1 == x2 {
            for y in y1.min(y2) ..= y2.max(y1) {
                *grid.entry((x1, y)).or_default() += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2) ..= x2.max(x1) {
                *grid.entry((x, y1)).or_default() += 1;
            }
        } else {
            let x_diff = x1 - x2;
            let y_diff = y1 - y2;

            for i in 0..x_diff.abs() + 1 {
                let add_x = if x_diff < 0 { i } else { i * -1 };
                let add_y = if y_diff < 0 { i } else { i * -1 };
                *grid.entry((x1 + add_x, y1 + add_y)).or_default() += 1;
            }
        }
    }

    grid.values().filter(|v| **v >= 2).count()
}


#[test]
fn verify_part_two() {
    assert_eq!(part_two(
        vec![
            "0,9 -> 5,9".to_owned(),
            "8,0 -> 0,8".to_owned(),
            "9,4 -> 3,4".to_owned(),
            "2,2 -> 2,1".to_owned(),
            "7,0 -> 7,4".to_owned(),
            "6,4 -> 2,0".to_owned(),
            "0,9 -> 2,9".to_owned(),
            "3,4 -> 1,4".to_owned(),
            "0,0 -> 8,8".to_owned(),
            "5,5 -> 8,2".to_owned(),
        ]
    ), 12);
}