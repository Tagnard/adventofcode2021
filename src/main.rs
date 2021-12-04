use aoc_2021::*;
use aoc_2021::Input;

fn main() {
    println!("Day 1 - Part 1: {}", day::one::part_one(Input::as_u32("./input/day_1_part_1.txt")));
    println!("Day 1 - Part 2: {}", day::one::part_two(Input::as_u32("./input/day_1_part_2.txt")));
    println!("Day 2 - Part 1: {}", day::two::part_one(Input::as_string("./input/day_2_part_1.txt")));
    println!("Day 2 - Part 2: {}", day::two::part_two(Input::as_string("./input/day_2_part_2.txt")));
    println!("Day 3 - Part 1: {}", day::three::part_one(Input::as_string("./input/day_3_part_1.txt")));
    println!("Day 3 - Part 2: {}", day::three::part_two(Input::as_string("./input/day_3_part_2.txt")));
    println!("Day 4 - Part 1: {}", day::four::part_one(Input::as_string("./input/day_4_part_1.txt")));
    println!("Day 4 - Part 2: {}", day::four::part_two(Input::as_string("./input/day_4_part_2.txt")));
}