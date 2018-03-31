extern crate day02_part_1;

const PUZZLE_INPUT: &'static str = include_str!("../day02_input.txt");

fn main() {
    let solution = day02_part_1::aoc_day02_part_1(PUZZLE_INPUT);
    println!("PUZZLE_INPUT: {}", PUZZLE_INPUT);
    println!("solution: {}", solution);
}
