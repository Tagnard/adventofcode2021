pub fn part_one(input: Vec<String>) -> u32 {
    let input_len = input[0].len() as u32;
    let input: Vec<u32> = input.iter().map(|e| u32::from_str_radix(&e, 2).unwrap()).collect();
    
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for i in (0..input_len).rev() {
        let ones = input.iter().filter(|x| (*x & 2u32.pow(i)) == 2u32.pow(i)).count();

        if ones > input.len() - ones {
            gamma |= 2u32.pow(i);
        } else {
            epsilon |= 2u32.pow(i);
        }
    }

    gamma * epsilon
}

#[test]
fn verify_part_one() {
    assert_eq!(part_one(vec![
        "00100".to_owned(),
        "11110".to_owned(),
        "10110".to_owned(),
        "10111".to_owned(),
        "10101".to_owned(),
        "01111".to_owned(),
        "00111".to_owned(),
        "11100".to_owned(),
        "10000".to_owned(),
        "11001".to_owned(),
        "00010".to_owned(),
        "01010".to_owned(),
    ]), 198);
}

pub fn part_two(input: Vec<String>) -> u32 {
    let input_len = input[0].len() as u32;
    let input: Vec<u32> = input.iter().map(|e| u32::from_str_radix(&e, 2).unwrap()).collect();

    let most_common = |input: Vec<u32>| -> u32 {
        let mut matches: Vec<u32> = input;
        for i in (0..input_len).rev() {
            if matches.iter().count() > 1 {
                let ones = matches.iter().filter(|&&x| (x & 2u32.pow(i)) == 2u32.pow(i)).count();
                let zeros = matches.len() - ones;
                if ones == zeros || ones > zeros {
                    matches = matches.into_iter().filter(|x| x & 2u32.pow(i) == 2u32.pow(i)).collect();
                } else {
                    matches = matches.into_iter().filter(|x| x & 2u32.pow(i) == 0).collect();
                }
            } else {
                break;
            }
        }

        matches[0]
    };

    let least_common = |input: Vec<u32>| -> u32 {
        let mut matches: Vec<u32> = input;
        for i in (0..input_len).rev() {
            if matches.iter().count() > 1 {
                let ones = matches.iter().filter(|&&x| (x & 2u32.pow(i)) == 2u32.pow(i)).count();
                let zeros = matches.len() - ones;
                if ones == zeros || ones > zeros {
                    matches = matches.into_iter().filter(|x| x & 2u32.pow(i) == 0).collect();
                } else {
                    matches = matches.into_iter().filter(|x| x & 2u32.pow(i) == 2u32.pow(i)).collect();
                }
            } else {
                break;
            }
        }

        matches[0]
    };

    most_common(input.clone()) * least_common(input.clone())
}


#[test]
fn verify_part_two() {
    assert_eq!(part_two(vec![
        "00100".to_owned(),
        "11110".to_owned(),
        "10110".to_owned(),
        "10111".to_owned(),
        "10101".to_owned(),
        "01111".to_owned(),
        "00111".to_owned(),
        "11100".to_owned(),
        "10000".to_owned(),
        "11001".to_owned(),
        "00010".to_owned(),
        "01010".to_owned(),
    ]), 230);
}