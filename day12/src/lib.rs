//! # Day 12:
//!
//! ## Part One
//!
//! ## Part Two
//!

pub struct Solution {
    pub part1: u64,
    pub part2: u64,
}

pub fn aoc_day12(input: &str) -> Solution {}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day12 {
            // const PUZZLE_INPUT: &'static str = include_str!("../input");

            mod part1 {

                /*
                mod solution {
                    use ::*;
                    use super::super::PUZZLE_INPUT;

                    #[test]
                    fn solution() {
                        let expected = 0;
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day12(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }
                */

                mod given {
                    use ::*;

                    #[test]
                    fn ex01_() {
                        let expected = 6;
                        let input = "0 <-> 2
                                     1 <-> 1
                                     2 <-> 0, 3, 4
                                     3 <-> 2, 4
                                     4 <-> 2, 3, 6
                                     5 <-> 6
                                     6 <-> 4, 5";
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day12(input);

                        assert_eq!(expected, to_check);
                    }
                }

                /*
                mod extra {
                    use ::*;
                }
                */
            }

            /*
            mod part2 {
                mod solution {
                    use ::*;
                    use super::super::PUZZLE_INPUT;

                    #[test]
                    fn solution() {
                        let expected = 0;
                        let Solution {
                            part1: _,
                            part2: to_check,
                        } = aoc_day12(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use ::*;
                }

                mod extra {
                    use ::*;
                }
            }
            */
        }
    }
}
