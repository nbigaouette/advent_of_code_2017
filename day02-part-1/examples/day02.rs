extern crate day02;

const PUZZLE_INPUT: &'static str = include_str!("../day02_input.txt");

fn main() {
    let solution = day02::aoc_day02(PUZZLE_INPUT);
    println!("PUZZLE_INPUT: {}", PUZZLE_INPUT);
    println!("solution: {}", solution);
}
