pub fn part_one(input: Vec<String>) -> i64 {
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
fn verify_part_one() {
    assert_eq!(part_one(vec![
        "forward 5".to_owned(), 
        "down 5".to_owned(), 
        "forward 8".to_owned(), 
        "up 3".to_owned(), 
        "down 8".to_owned(), 
        "forward 2".to_owned()]), 150);
}

pub fn part_two(input: Vec<String>) -> i64 {
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
fn verify_part_two() {
    assert_eq!(part_two(vec![
        "forward 5".to_owned(), 
        "down 5".to_owned(), 
        "forward 8".to_owned(), 
        "up 3".to_owned(), 
        "down 8".to_owned(), 
        "forward 2".to_owned()]), 900);
}