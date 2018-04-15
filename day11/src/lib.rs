//! # Day 11: Hex Ed
//!
//! Crossing the bridge, you've barely reached the other side of the stream when a program comes up to you, clearly in distress. "It's my child process," she says, "he's gotten lost in an infinite grid!"
//!
//! Fortunately for her, you have plenty of experience with infinite grids.
//!
//! Unfortunately for you, it's a [hex grid](https://en.wikipedia.org/wiki/Hexagonal_tiling).
//!
//! The hexagons ("hexes") in this grid are aligned such that adjacent hexes can be found to the north, northeast, southeast, south, southwest, and northwest:
//!
//!```text
//!       \ n  /
//!     nw +--+ ne
//!       /    \
//!     -+      +-
//!       \    /
//!     sw +--+ se
//!       / s  \
//!```
//!
//!
//! You have the path the child process took. Starting where he started, you need to determine the fewest number of steps required to reach him. (A "step" means to move from the hex you are in to any adjacent hex.)
//!
//! For example:
//!
//! *   `ne,ne,ne` is `3` steps away.
//! *   `ne,ne,sw,sw` is `0` steps away (back where you started).
//! *   `ne,ne,s,s` is `2` steps away (`se,se`).
//! *   `se,sw,se,sw,sw` is `3` steps away (`s,s,sw`).
//!
//! ## Part Two
//!
//! How many steps away is the furthest he ever got from his starting position?
//!

pub struct Solution {
    pub part1: u64,
    pub part2: u64,
}

pub enum Move {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
}

impl Move {
    pub fn from_str(input: &str) -> Move {
        match input {
            "n" => Move::North,
            "ne" => Move::NorthEast,
            "nw" => Move::NorthWest,
            "s" => Move::South,
            "se" => Move::SouthEast,
            "sw" => Move::SouthWest,
            _ => panic!(format!("Unsupported input '{}'", input)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    x: i64,
    y: i64,
    furthest: u64,
}

impl Position {
    pub fn origin() -> Position {
        Position {
            x: 0,
            y: 0,
            furthest: 0,
        }
    }

    pub fn advance(&mut self, by: &Move) {
        match by {
            &Move::North => {
                self.y += 2;
            }
            &Move::NorthEast => {
                self.x += 1;
                self.y += 1;
            }
            &Move::NorthWest => {
                self.x -= 1;
                self.y += 1;
            }
            &Move::South => {
                self.y -= 2;
            }
            &Move::SouthEast => {
                self.x += 1;
                self.y -= 1;
            }
            &Move::SouthWest => {
                self.x -= 1;
                self.y -= 1;
            }
        }

        self.furthest = self.furthest.max(self.fewest_nb_steps());
    }

    pub fn fewest_nb_steps(&self) -> u64 {
        let nb_diag = self.x.abs();
        let nb_vert = ((self.y.abs() - nb_diag) / 2).max(0);

        (nb_diag + nb_vert) as u64
    }
}

pub fn aoc_day11(input: &str) -> Solution {
    let mut position = Position::origin();

    for step in input.trim().split(',') {
        let parsed_move = Move::from_str(step);
        position.advance(&parsed_move);
    }

    Solution {
        part1: position.fewest_nb_steps(),
        part2: position.furthest,
    }
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day11 {
            const PUZZLE_INPUT: &'static str = include_str!("../input");

            mod part1 {

                mod solution {
                    use ::*;
                    use super::super::PUZZLE_INPUT;

                    #[test]
                    fn solution() {
                        let expected = 810;
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day11(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use ::*;

                    #[test]
                    fn ex01_ne_ne_ne() {
                        let expected = 3;
                        let input = "ne,ne,ne";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day11(input);

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex02_ne_ne_sw_sw() {
                        let expected = 0;
                        let input = "ne,ne,sw,sw";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day11(input);

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex03_ne_ne_s_s() {
                        let expected = 2;
                        let input = "ne,ne,s,s";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day11(input);

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ex04_se_sw_se_sw_sw() {
                        let expected = 3;
                        let input = "se,sw,se,sw,sw";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day11(input);

                        assert_eq!(expected, to_check);
                    }
                }

                mod extra {
                    use ::*;

                    #[test]
                    fn ne_sw_ne_sw() {
                        let expected = 0;
                        let input = "ne,sw,ne,sw";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day11(input);

                        assert_eq!(expected, to_check);
                    }
                }
            }

            mod part2 {
                mod given {}

                mod extra {
                    use ::*;

                    #[test]
                    fn ne_sw_ne_sw() {
                        let expected = 1;
                        let input = "ne,sw,ne,sw";
                        let Solution {
                            part1: _,
                            part2: to_check,
                        } = aoc_day11(input);

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn ne_ne_sw_sw() {
                        let expected = 2;
                        let input = "ne,ne,sw,sw";
                        let Solution {
                            part1: _,
                            part2: to_check,
                        } = aoc_day11(input);

                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn nw_sw_sw_s_s_ne_ne() {
                        let expected = 4;
                        let input = "nw,sw,sw,s,s,ne,ne";
                        let Solution {
                            part1: _,
                            part2: to_check,
                        } = aoc_day11(input);

                        assert_eq!(expected, to_check);
                    }
                }

                mod solution {
                    use ::*;
                    use super::super::PUZZLE_INPUT;

                    #[test]
                    fn solution() {
                        let expected = 1567;
                        let Solution {
                            part1: _,
                            part2: to_check,
                        } = aoc_day11(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }
            }
        }
    }
}
