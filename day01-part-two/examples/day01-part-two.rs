extern crate day01_part_two;

const PUZZLE_INPUT: &'static str = include_str!("../day01_input.txt");

fn main() {
    let solution = day01_part_two::aoc_day01_part_two(PUZZLE_INPUT);
    println!("PUZZLE_INPUT: {}", PUZZLE_INPUT);
    println!("solution: {}", solution);
}
