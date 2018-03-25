extern crate day01_part_1;

const PUZZLE_INPUT: &'static str = include_str!("../day01_input.txt");

fn main() {
    let solution = day01_part_1::aoc_day01(PUZZLE_INPUT);
    println!("PUZZLE_INPUT: {}", PUZZLE_INPUT);
    println!("solution: {}", solution);
}
