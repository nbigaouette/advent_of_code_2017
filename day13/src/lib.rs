//! # Day 13: Packet Scanners
//!
//! ## Part One
//!
//! You need to cross a vast _firewall_. The firewall consists of several layers, each with
//! a _security scanner_ that moves back and forth across the layer. To succeed, you must
//! not be detected by a scanner.
//!
//! By studying the firewall briefly, you are able to record (in your puzzle input) the
//! _depth_ of each layer and the _range_ of the scanning area for the scanner within it,
//! written as `depth: range`. Each layer has a thickness of exactly `1`. A layer at depth
//! `0` begins immediately inside the firewall; a layer at depth `1` would start
//! immediately after that.
//!
//! For example, suppose you've recorded the following:
//!
//!```text
//!     0: 3
//!     1: 2
//!     4: 4
//!     6: 4
//!```  
//!
//! This means that there is a layer immediately inside the firewall (with range `3`), a
//! second layer immediately after that (with range `2`), a third layer which begins at
//! depth `4` (with range `4`), and a fourth layer which begins at depth 6 (also with
//! range `4`). Visually, it might look like this:
//!
//!```text
//!      0   1   2   3   4   5   6
//!     [ ] [ ] ... ... [ ] ... [ ]
//!     [ ] [ ]         [ ]     [ ]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!```  
//!
//! Within each layer, a security scanner moves back and forth within its range. Each
//! security scanner starts at the top and moves down until it reaches the bottom,
//! then moves up until it reaches the top, and repeats. A security scanner takes
//! _one picosecond_ to move one step. Drawing scanners as `S`, the first few
//! picoseconds look like this:
//!
//!```text
//!     Picosecond 0:
//!      0   1   2   3   4   5   6
//!     [S] [S] ... ... [S] ... [S]
//!     [ ] [ ]         [ ]     [ ]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!     Picosecond 1:
//!      0   1   2   3   4   5   6
//!     [ ] [ ] ... ... [ ] ... [ ]
//!     [S] [S]         [S]     [S]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!     Picosecond 2:
//!      0   1   2   3   4   5   6
//!     [ ] [S] ... ... [ ] ... [ ]
//!     [ ] [ ]         [ ]     [ ]
//!     [S]             [S]     [S]
//!                     [ ]     [ ]
//!     
//!     Picosecond 3:
//!      0   1   2   3   4   5   6
//!     [ ] [ ] ... ... [ ] ... [ ]
//!     [S] [S]         [ ]     [ ]
//!     [ ]             [ ]     [ ]
//!                     [S]     [S]
//!```  
//!
//! Your plan is to hitch a ride on a packet about to move through the firewall. The
//! packet will travel along the top of each layer, and it moves at _one layer per
//! picosecond_. Each picosecond, the packet moves one layer forward (its first move
//! takes it into layer 0), and then the scanners move one step. If there is a scanner
//! at the top of the layer _as your packet enters it_, you are _caught_. (If a
//! scanner moves into the top of its layer while you are there, you are _not_
//! caught: it doesn't have time to notice you before you leave.) If you were to
//! do this in the configuration above, marking your current position with parentheses,
//! your passage through the firewall would look like this:
//!
//!```text
//!     Initial state:
//!      0   1   2   3   4   5   6
//!     [S] [S] ... ... [S] ... [S]
//!     [ ] [ ]         [ ]     [ ]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!     Picosecond 0:
//!      0   1   2   3   4   5   6
//!     (S) [S] ... ... [S] ... [S]
//!     [ ] [ ]         [ ]     [ ]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     ( ) [ ] ... ... [ ] ... [ ]
//!     [S] [S]         [S]     [S]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!     
//!     Picosecond 1:
//!      0   1   2   3   4   5   6
//!     [ ] ( ) ... ... [ ] ... [ ]
//!     [S] [S]         [S]     [S]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     [ ] (S) ... ... [ ] ... [ ]
//!     [ ] [ ]         [ ]     [ ]
//!     [S]             [S]     [S]
//!                     [ ]     [ ]
//!     
//!     
//!     Picosecond 2:
//!      0   1   2   3   4   5   6
//!     [ ] [S] (.) ... [ ] ... [ ]
//!     [ ] [ ]         [ ]     [ ]
//!     [S]             [S]     [S]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     [ ] [ ] (.) ... [ ] ... [ ]
//!     [S] [S]         [ ]     [ ]
//!     [ ]             [ ]     [ ]
//!                     [S]     [S]
//!     
//!     
//!     Picosecond 3:
//!      0   1   2   3   4   5   6
//!     [ ] [ ] ... (.) [ ] ... [ ]
//!     [S] [S]         [ ]     [ ]
//!     [ ]             [ ]     [ ]
//!                     [S]     [S]
//!     
//!      0   1   2   3   4   5   6
//!     [S] [S] ... (.) [ ] ... [ ]
//!     [ ] [ ]         [ ]     [ ]
//!     [ ]             [S]     [S]
//!                     [ ]     [ ]
//!     
//!     
//!     Picosecond 4:
//!      0   1   2   3   4   5   6
//!     [S] [S] ... ... ( ) ... [ ]
//!     [ ] [ ]         [ ]     [ ]
//!     [ ]             [S]     [S]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     [ ] [ ] ... ... ( ) ... [ ]
//!     [S] [S]         [S]     [S]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!     
//!     Picosecond 5:
//!      0   1   2   3   4   5   6
//!     [ ] [ ] ... ... [ ] (.) [ ]
//!     [S] [S]         [S]     [S]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     [ ] [S] ... ... [S] (.) [S]
//!     [ ] [ ]         [ ]     [ ]
//!     [S]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!     
//!     Picosecond 6:
//!      0   1   2   3   4   5   6
//!     [ ] [S] ... ... [S] ... (S)
//!     [ ] [ ]         [ ]     [ ]
//!     [S]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     [ ] [ ] ... ... [ ] ... ( )
//!     [S] [S]         [S]     [S]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!```
//!
//! In this situation, you are _caught_ in layers `0` and `6`, because your packet
//! entered the layer when its scanner was at the top when you entered it. You are
//! _not_ caught in layer `1`, since the scanner moved into the top of the layer
//! once you were already there.
//!
//! The _severity_ of getting caught on a layer is equal to its _depth_ multiplied
//! by its _range_. (Ignore layers in which you do not get caught.) The severity
//! of the whole trip is the sum of these values. In the example above, the trip
//! severity is `0*3 + 6*4 = _24_`.
//!
//! Given the details of the firewall you've recorded, if you leave immediately,
//! _what is the severity of your whole trip_?
//!
//!
//! ## Part Two
//!
//! Now, you need to pass through the firewall without being caught - easier said than done.
//!
//! You can't control the speed of the packet, but you can _delay_ it any number of picoseconds. For each picosecond you delay the packet before beginning your trip, all security scanners move one step. You're not in the firewall during this time; you don't enter layer `0` until you stop delaying the packet.
//!
//! In the example above, if you delay `10` picoseconds (picoseconds `0` \- `9`), you won't get caught:
//!
//!```text
//!     State after delaying:
//!      0   1   2   3   4   5   6
//!     [ ] [S] ... ... [ ] ... [ ]
//!     [ ] [ ]         [ ]     [ ]
//!     [S]             [S]     [S]
//!                     [ ]     [ ]
//!     
//!     Picosecond 10:
//!      0   1   2   3   4   5   6
//!     ( ) [S] ... ... [ ] ... [ ]
//!     [ ] [ ]         [ ]     [ ]
//!     [S]             [S]     [S]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     ( ) [ ] ... ... [ ] ... [ ]
//!     [S] [S]         [S]     [S]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!     
//!     Picosecond 11:
//!      0   1   2   3   4   5   6
//!     [ ] ( ) ... ... [ ] ... [ ]
//!     [S] [S]         [S]     [S]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     [S] (S) ... ... [S] ... [S]
//!     [ ] [ ]         [ ]     [ ]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!     
//!     Picosecond 12:
//!      0   1   2   3   4   5   6
//!     [S] [S] (.) ... [S] ... [S]
//!     [ ] [ ]         [ ]     [ ]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     [ ] [ ] (.) ... [ ] ... [ ]
//!     [S] [S]         [S]     [S]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!     
//!     Picosecond 13:
//!      0   1   2   3   4   5   6
//!     [ ] [ ] ... (.) [ ] ... [ ]
//!     [S] [S]         [S]     [S]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     [ ] [S] ... (.) [ ] ... [ ]
//!     [ ] [ ]         [ ]     [ ]
//!     [S]             [S]     [S]
//!                     [ ]     [ ]
//!     
//!     
//!     Picosecond 14:
//!      0   1   2   3   4   5   6
//!     [ ] [S] ... ... ( ) ... [ ]
//!     [ ] [ ]         [ ]     [ ]
//!     [S]             [S]     [S]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     [ ] [ ] ... ... ( ) ... [ ]
//!     [S] [S]         [ ]     [ ]
//!     [ ]             [ ]     [ ]
//!                     [S]     [S]
//!     
//!     
//!     Picosecond 15:
//!      0   1   2   3   4   5   6
//!     [ ] [ ] ... ... [ ] (.) [ ]
//!     [S] [S]         [ ]     [ ]
//!     [ ]             [ ]     [ ]
//!                     [S]     [S]
//!     
//!      0   1   2   3   4   5   6
//!     [S] [S] ... ... [ ] (.) [ ]
//!     [ ] [ ]         [ ]     [ ]
//!     [ ]             [S]     [S]
//!                     [ ]     [ ]
//!     
//!     
//!     Picosecond 16:
//!      0   1   2   3   4   5   6
//!     [S] [S] ... ... [ ] ... ( )
//!     [ ] [ ]         [ ]     [ ]
//!     [ ]             [S]     [S]
//!                     [ ]     [ ]
//!     
//!      0   1   2   3   4   5   6
//!     [ ] [ ] ... ... [ ] ... ( )
//!     [S] [S]         [S]     [S]
//!     [ ]             [ ]     [ ]
//!                     [ ]     [ ]
//!```text
//!
//! Because all smaller delays would get you caught, the fewest number of picoseconds you would need to delay to get through safely is `10`.
//!
//! _What is the fewest number of picoseconds_ that you need to delay the packet to pass through the firewall without being caught?
//!

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
#[macro_use]
extern crate indoc;
extern crate rayon;

pub mod firewall {
    use std;

    #[derive(Debug)]
    pub struct Firewall {
        pub layers: Vec<Scanner>,
        pub step: isize,
    }

    #[derive(Clone, Debug, Default)]
    pub struct Scanner {
        pub depth: usize,
        pub loc: isize,
        increment: isize,
    }

    impl Scanner {
        pub fn step(&mut self) {
            if self.depth > 0 {
                if self.loc == 0 || self.loc == self.depth as isize - 1 {
                    self.increment *= -1;
                }
                self.loc += self.increment;
            }
        }
    }

    impl Firewall {
        pub fn from_str(input: &str) -> Firewall {
            let layers_tmp: Vec<(usize, usize)> = input
                .lines()
                .map(|line| {
                    let elements: Vec<_> = line.trim().split(": ").collect();
                    (
                        elements[0].parse().expect("integer"),
                        elements[1].parse().expect("integer"),
                    )
                })
                .collect();

            let max_depth = layers_tmp.last().expect("at least one element").0 + 1;
            let mut layers: Vec<Scanner> = vec![std::default::Default::default(); max_depth];

            for layer in &layers_tmp {
                layers[layer.0].depth = layer.1;
                layers[layer.0].increment = -1;
            }

            Firewall {
                layers: layers,
                step: 0,
            }
        }

        pub fn to_string(&self) -> String {
            let mut s = format!("Picosecond {}:\n", self.step);
            self.layers
                .iter()
                .enumerate()
                .for_each(|(i, _)| s.push_str(&format!(" {:1}  ", i)));
            // Remove two trailing spaces
            s.pop();
            s.pop();
            s.push('\n');

            let max_depth = self.layers
                .iter()
                .fold(0, |acc, scanner| acc.max(scanner.depth));

            for level in 0..max_depth {
                self.layers.iter().for_each(|scanner| {
                    if scanner.depth == 0 && level == 0 {
                        s.push_str("... ");
                    } else {
                        if level >= scanner.depth {
                            s.push_str("    ");
                        } else {
                            s.push('[');
                            if scanner.loc == level as isize {
                                s.push('S');
                            } else {
                                s.push(' ');
                            }
                            s.push_str("] ");
                        }
                    }
                });
                // Remove trailing space
                s.pop();
                s.push('\n');
            }

            // Remove trailing newline
            s.pop();

            s
        }

        pub fn step(&mut self) {
            self.layers.iter_mut().for_each(|scanner| scanner.step());
            self.step += 1;
        }
    }
}

#[derive(Debug)]
pub struct FirewallHopper {
    fw: firewall::Firewall,
    packet_location: isize,
    step_severity: usize,
    got_caught: bool,
}

impl FirewallHopper {
    pub fn from_str(input: &str) -> FirewallHopper {
        FirewallHopper::with_delay(input, 0)
    }

    pub fn with_delay(input: &str, delay: usize) -> FirewallHopper {
        let fw = firewall::Firewall::from_str(input);

        let mut hopper = FirewallHopper {
            fw: fw,
            packet_location: -1 - (delay as isize),
            step_severity: 0,
            got_caught: false,
        };
        hopper.fw.step = -1;
        hopper.step();
        hopper
    }

    pub fn step(&mut self) {
        // Reset flag
        self.got_caught = false;

        // Step 1: Packet move
        self.packet_location += 1;

        // Check detection
        self.step_severity = self.calculate_step_severity();

        // Step 2: Scanners move
        self.fw.step();
    }

    fn will_get_caught(&mut self) -> bool {
        while let Some(_severity) = self.next() {
            if self.got_caught {
                return true;
            }
        }

        return false;
    }

    fn calculate_step_severity(&mut self) -> usize {
        if self.packet_location < 0 {
            0
        } else {
            let i = self.packet_location as usize;

            if i >= self.fw.layers.len() {
                0
            } else {
                if self.fw.layers[i].depth > 0 && self.fw.layers[i].loc == 0 {
                    // If scanner is at top location where the packet is...
                    self.got_caught = true;
                    self.fw.layers[i].depth * i
                } else {
                    0
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        let fw_s = self.fw.to_string();

        // Don't forget the +1 for the newlines
        let first_line_length = fw_s.lines().nth(0).unwrap().len() + 1;
        let second_line_length = fw_s.lines().nth(1).unwrap().len() + 1;

        let mut s = fw_s.as_bytes().to_vec();

        if self.packet_location >= 0 {
            let packet_location = self.packet_location as usize;
            let i = first_line_length + second_line_length + 4 * (packet_location + 1) - 3;

            s[i - 1] = b'(';
            s[i + 1] = b')';
        }

        let s = String::from_utf8(s).expect("UTF-8 string");

        s
    }
}

impl Iterator for FirewallHopper {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.packet_location >= self.fw.layers.len() as isize {
            None
        } else {
            self.step();
            Some(self.step_severity)
        }
    }
}

pub mod part1 {
    use *;

    pub fn aoc_day13(input: &str) -> usize {
        let hopper = FirewallHopper::from_str(input);
        hopper.sum()
    }
}

pub mod part2 {
    use *;

    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    pub fn aoc_day13(input: &str) -> usize {
        let part2_min_delay = (0_usize..10_000_000)
            .into_par_iter()
            .find_first(|&delay| {
                let mut hopper = FirewallHopper::with_delay(input, delay);
                !hopper.will_get_caught()
            })
            .unwrap();

        part2_min_delay
    }
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day13 {
            const PUZZLE_INPUT: &'static str = include_str!("../input");
            const EXAMPLE_INPUT: &'static str = "0: 3
                                                 1: 2
                                                 4: 4
                                                 6: 4";

            mod part1 {

                mod solution {
                    use super::super::PUZZLE_INPUT;
                    use *;

                    #[test]
                    fn solution() {
                        let expected = 1504;
                        let to_check = part1::aoc_day13(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use super::super::EXAMPLE_INPUT;
                    use *;

                    #[test]
                    fn ex01() {
                        let expected = 24;
                        let to_check = part1::aoc_day13(EXAMPLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod extra {
                    use super::super::EXAMPLE_INPUT;
                    use *;

                    #[test]
                    fn representation_firewall() {
                        let mut fw = firewall::Firewall::from_str(EXAMPLE_INPUT);
                        let expected = indoc!(
                            "Picosecond 0:
                             0   1   2   3   4   5   6
                            [S] [S] ... ... [S] ... [S]
                            [ ] [ ]         [ ]     [ ]
                            [ ]             [ ]     [ ]
                                            [ ]     [ ]"
                        );
                        let to_check = fw.to_string();
                        assert_eq!(expected, to_check);

                        fw.step();
                        let expected = indoc!(
                            "Picosecond 1:
                                         0   1   2   3   4   5   6
                                        [ ] [ ] ... ... [ ] ... [ ]
                                        [S] [S]         [S]     [S]
                                        [ ]             [ ]     [ ]
                                                        [ ]     [ ]"
                        );
                        let to_check = fw.to_string();
                        assert_eq!(expected, to_check);

                        fw.step();
                        let expected = indoc!(
                            "Picosecond 2:
                                         0   1   2   3   4   5   6
                                        [ ] [S] ... ... [ ] ... [ ]
                                        [ ] [ ]         [ ]     [ ]
                                        [S]             [S]     [S]
                                                        [ ]     [ ]"
                        );
                        let to_check = fw.to_string();
                        assert_eq!(expected, to_check);

                        fw.step();
                        let expected = indoc!(
                            "Picosecond 3:
                                         0   1   2   3   4   5   6
                                        [ ] [ ] ... ... [ ] ... [ ]
                                        [S] [S]         [ ]     [ ]
                                        [ ]             [ ]     [ ]
                                                        [S]     [S]"
                        );
                        let to_check = fw.to_string();
                        assert_eq!(expected, to_check);
                    }

                    #[test]
                    fn representation_hopper() {
                        let mut hopper = FirewallHopper::from_str(EXAMPLE_INPUT);
                        assert_eq!(hopper.got_caught, true);
                        let expected = indoc!(
                            "Picosecond 0:
                             0   1   2   3   4   5   6
                            ( ) [ ] ... ... [ ] ... [ ]
                            [S] [S]         [S]     [S]
                            [ ]             [ ]     [ ]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);
                        assert_eq!(hopper.got_caught, true);

                        hopper.step();
                        let expected = indoc!(
                            "Picosecond 1:
                             0   1   2   3   4   5   6
                            [ ] (S) ... ... [ ] ... [ ]
                            [ ] [ ]         [ ]     [ ]
                            [S]             [S]     [S]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);
                        assert_eq!(hopper.got_caught, false);

                        hopper.step();
                        let expected = indoc!(
                            "Picosecond 2:
                             0   1   2   3   4   5   6
                            [ ] [ ] (.) ... [ ] ... [ ]
                            [S] [S]         [ ]     [ ]
                            [ ]             [ ]     [ ]
                                            [S]     [S]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);
                        assert_eq!(hopper.got_caught, false);

                        hopper.step();
                        let expected = indoc!(
                            "Picosecond 3:
                             0   1   2   3   4   5   6
                            [S] [S] ... (.) [ ] ... [ ]
                            [ ] [ ]         [ ]     [ ]
                            [ ]             [S]     [S]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);
                        assert_eq!(hopper.got_caught, false);

                        hopper.step();
                        let expected = indoc!(
                            "Picosecond 4:
                             0   1   2   3   4   5   6
                            [ ] [ ] ... ... ( ) ... [ ]
                            [S] [S]         [S]     [S]
                            [ ]             [ ]     [ ]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);
                        assert_eq!(hopper.got_caught, false);

                        hopper.step();
                        let expected = indoc!(
                            "Picosecond 5:
                             0   1   2   3   4   5   6
                            [ ] [S] ... ... [S] (.) [S]
                            [ ] [ ]         [ ]     [ ]
                            [S]             [ ]     [ ]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);
                        assert_eq!(hopper.got_caught, false);

                        hopper.step();
                        let expected = indoc!(
                            "Picosecond 6:
                             0   1   2   3   4   5   6
                            [ ] [ ] ... ... [ ] ... ( )
                            [S] [S]         [S]     [S]
                            [ ]             [ ]     [ ]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);
                        assert_eq!(hopper.got_caught, true);
                    }
                }
            }

            mod part2 {
                mod solution {
                    use super::super::PUZZLE_INPUT;
                    use *;

                    #[test]
                    #[ignore]
                    fn solution() {
                        // WARNING: This took 22 hours running in parallel on a
                        //          the 32 cores of an Intel(R) Xeon(R) CPU E5-2670 0 @ 2.60GHz
                        //          system.
                        //              real    1315m22.269s
                        //              user    42094m35.065s
                        //              sys     1m10.126s
                        //          As such, the test is not run automatically
                        //          (it's marked `#[ignore]`).
                        let expected = 3823370;
                        let to_check = part2::aoc_day13(PUZZLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod given {
                    use super::super::EXAMPLE_INPUT;
                    use *;

                    #[test]
                    fn ex01() {
                        let expected = 10;
                        let to_check = part2::aoc_day13(EXAMPLE_INPUT);

                        assert_eq!(expected, to_check);
                    }
                }

                mod extra {
                    use super::super::EXAMPLE_INPUT;
                    use *;

                    #[test]
                    fn representation_hopper() {
                        let mut hopper = FirewallHopper::with_delay(EXAMPLE_INPUT, 10);
                        let mut severity: usize = 0;

                        hopper.step(); // 1
                        severity += hopper.step_severity;
                        hopper.step(); // 2
                        severity += hopper.step_severity;
                        hopper.step(); // 3
                        severity += hopper.step_severity;
                        hopper.step(); // 4
                        severity += hopper.step_severity;
                        hopper.step(); // 5
                        severity += hopper.step_severity;
                        hopper.step(); // 6
                        severity += hopper.step_severity;
                        hopper.step(); // 7
                        severity += hopper.step_severity;
                        hopper.step(); // 8
                        severity += hopper.step_severity;

                        hopper.step(); // 9
                        severity += hopper.step_severity;
                        let expected = indoc!(
                            "Picosecond 9:
                              0   1   2   3   4   5   6
                             [ ] [S] ... ... [ ] ... [ ]
                             [ ] [ ]         [ ]     [ ]
                             [S]             [S]     [S]
                                             [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);

                        hopper.step(); // 10
                        severity += hopper.step_severity;
                        let expected = indoc!(
                            "Picosecond 10:
                             0   1   2   3   4   5   6
                            ( ) [ ] ... ... [ ] ... [ ]
                            [S] [S]         [S]     [S]
                            [ ]             [ ]     [ ]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);

                        hopper.step(); // 11
                        severity += hopper.step_severity;
                        let expected = indoc!(
                            "Picosecond 11:
                             0   1   2   3   4   5   6
                            [S] (S) ... ... [S] ... [S]
                            [ ] [ ]         [ ]     [ ]
                            [ ]             [ ]     [ ]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);

                        hopper.step(); // 12
                        severity += hopper.step_severity;
                        let expected = indoc!(
                            "Picosecond 12:
                             0   1   2   3   4   5   6
                            [ ] [ ] (.) ... [ ] ... [ ]
                            [S] [S]         [S]     [S]
                            [ ]             [ ]     [ ]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);

                        hopper.step(); // 13
                        severity += hopper.step_severity;
                        let expected = indoc!(
                            "Picosecond 13:
                             0   1   2   3   4   5   6
                            [ ] [S] ... (.) [ ] ... [ ]
                            [ ] [ ]         [ ]     [ ]
                            [S]             [S]     [S]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);

                        hopper.step(); // 14
                        severity += hopper.step_severity;
                        let expected = indoc!(
                            "Picosecond 14:
                             0   1   2   3   4   5   6
                            [ ] [ ] ... ... ( ) ... [ ]
                            [S] [S]         [ ]     [ ]
                            [ ]             [ ]     [ ]
                                            [S]     [S]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);

                        hopper.step(); // 15
                        severity += hopper.step_severity;
                        let expected = indoc!(
                            "Picosecond 15:
                             0   1   2   3   4   5   6
                            [S] [S] ... ... [ ] (.) [ ]
                            [ ] [ ]         [ ]     [ ]
                            [ ]             [S]     [S]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);

                        hopper.step(); // 16
                        severity += hopper.step_severity;
                        let expected = indoc!(
                            "Picosecond 16:
                             0   1   2   3   4   5   6
                            [ ] [ ] ... ... [ ] ... ( )
                            [S] [S]         [S]     [S]
                            [ ]             [ ]     [ ]
                                            [ ]     [ ]"
                        );
                        let to_check = hopper.to_string();
                        assert_eq!(expected, to_check);

                        assert_eq!(severity, 0);
                    }
                }
            }
        }
    }
}
