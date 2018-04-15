//! # Day 12: Digital Plumber
//!
//! ## Part One
//!
//! Walking along the memory banks of the stream, you find a small village that is experiencing
//! a little confusion: some programs can't communicate with each other.
//!
//! Programs in this village communicate using a fixed system of _pipes_. Messages are passed
//! between programs using these pipes, but most programs aren't connected to each other
//! directly. Instead, programs pass messages between each other until the message reaches
//! the intended recipient.
//!
//! For some reason, though, some of these messages aren't ever reaching their intended
//! recipient, and the programs suspect that some pipes are missing. They would like you
//! to investigate.
//!
//! You walk through the village and record the ID of each program and the IDs with which
//! it can communicate directly (your puzzle input). Each program has one or more programs
//! with which it can communicate, and these pipes are bidirectional; if `8` says it can
//! communicate with `11`, then `11` will say it can communicate with `8`.
//!
//! You need to figure out how many programs are in the group that contains program ID `0`.
//!
//! For example, suppose you go door-to-door like a travelling salesman and record the
//! following list:
//!
//!```text
//! 0 <-> 2
//! 1 <-> 1
//! 2 <-> 0, 3, 4
//! 3 <-> 2, 4
//! 4 <-> 2, 3, 6
//! 5 <-> 6
//! 6 <-> 4, 5
//!```
//!
//! In this example, the following programs are in the group that contains program ID `0`:
//!
//! * Program `0` by definition.
//! * Program `2`, directly connected to program `0`.
//! * Program `3` via program `2`.
//! * Program `4` via program `2`.
//! * Program `5` via programs `6`, then `4`, then `2`.
//! * Program `6` via programs `4`, then `2`.
//!
//! Therefore, a total of `6` programs are in this group; all but program `1`, which
//! has a pipe that connects it to itself.
//!
//! _How many programs_ are in the group that contains program ID `0`?
//!
//! ## Part Two
//!
//! There are more programs than just the ones in the group containing program ID `0`. The
//! rest of them have no way of reaching that group, and still might have no way of reaching
//! each other.
//!
//! A _group_ is a collection of programs that can all communicate via pipes either directly
//! or indirectly. The programs you identified just a moment ago are all part of the same
//! group. Now, they would like you to determine the total number of groups.
//!
//! In the example above, there were `2` groups: one consisting of programs `0,2,3,4,5,6`,
//! and the other consisting solely of program `1`.
//!
//! _How many groups are there_ in total?
//!
//!

use std::collections::HashSet;
use std::collections::HashMap;

pub struct Solution {
    pub part1: usize,
    pub part2: u64,
}

struct Connections<'a> {
    connections: HashMap<&'a str, HashSet<&'a str>>,
    unvisited: HashMap<&'a str, HashSet<&'a str>>,
    groups: HashMap<&'a str, HashSet<&'a str>>,
}

impl<'a> Connections<'a> {
    fn from_str(input: &'a str) -> Connections {
        let connections: HashMap<&'a str, HashSet<&'a str>> = input
            .trim()
            .lines()
            .map(|line| {
                let mut s = line.split(" <-> ");
                let pid = s.next().unwrap().trim();
                let connected_to_those_pids = s.next().unwrap().split(", ").collect();
                (pid, connected_to_those_pids)
            })
            .collect();

        Connections {
            connections: connections.clone(),
            unvisited: connections,
            groups: HashMap::new(),
        }
    }

    fn build_group_pid_0(&mut self) {
        let mut to_visit = HashSet::<&str>::new();

        self.unvisited.remove("0").unwrap().iter().for_each(|pid| {
            to_visit.insert(pid);
        });
        self.groups.insert("0", HashSet::new());

        while to_visit.len() > 0 {
            for pid in to_visit.iter() {
                self.groups.get_mut("0").unwrap().insert(pid);
            }

            let to_visit_new = to_visit.clone();
            to_visit.clear();
            for pid in to_visit_new.iter() {
                for pid_to_insert in &self.connections[pid] {
                    if !self.groups["0"].contains(pid_to_insert) {
                        to_visit.insert(pid_to_insert);
                    }
                }
            }
        }
    }
}

pub fn aoc_day12(input: &str) -> Solution {
    let mut connections = Connections::from_str(input);
    connections.build_group_pid_0();

    Solution {
        part1: connections.groups["0"].len(),
        part2: 0,
    }
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day12 {
            const PUZZLE_INPUT: &'static str = include_str!("../input");

            mod part1 {

                mod solution {
                    use ::*;
                    use super::super::PUZZLE_INPUT;

                    #[test]
                    fn solution() {
                        let expected = 152;
                        let Solution {
                            part1: to_check,
                            part2: _,
                        } = aoc_day12(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use ::*;

                    #[test]
                    fn ex01() {
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
