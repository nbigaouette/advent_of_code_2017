extern crate day01;

const PUZZLE_INPUT: &'static str = include_str!("../day01_input.txt");

fn main() {
    let solution = day01::aoc_day01(PUZZLE_INPUT);
    println!("PUZZLE_INPUT: {}", PUZZLE_INPUT);
    println!("solution: {}", solution);
}
