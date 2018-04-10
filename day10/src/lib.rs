//! # Day 10: Knot Hash
//!
//! ## Part One
//!
//! You come across some programs that are trying to implement a software emulation of a hash based on knot-tying. The hash these programs are implementing isn't very strong, but you decide to help them anyway. You make a mental note to remind the Elves later not to invent their own cryptographic functions.
//!
//! This hash function simulates tying a knot in a circle of string with 256 marks on it. Based on the input to be hashed, the function repeatedly selects a span of string, brings the ends together, and gives the span a half-twist to reverse the order of the marks within it. After doing this many times, the order of the marks is used to build the resulting hash.
//!
//!```text
//!       4--5   pinch   4  5           4   1
//!      /    \  5,0,1  / \/ \  twist  / \ / \
//!     3      0  -->  3      0  -->  3   X   0
//!      \    /         \ /\ /         \ / \ /
//!       2--1           2  1           2   5
//!```
//!
//! To achieve this, begin with a _list_ of numbers from `0` to `255`, a _current position_ which begins at `0` (the first element in the list), a _skip size_ (which starts at `0`), and a sequence of _lengths_ (your puzzle input). Then, for each length:
//!
//! *   _Reverse_ the order of that _length_ of elements in the _list_, starting with the element at the _current position_.
//! *   _Move_ the _current position_ forward by that _length_ plus the _skip size_.
//! *   _Increase_ the _skip size_ by one.
//!
//! The _list_ is circular; if the _current position_ and the _length_ try to reverse elements beyond the end of the list, the operation reverses using as many extra elements as it needs from the front of the list. If the _current position_ moves past the end of the list, it wraps around to the front. _Lengths_ larger than the size of the _list_ are invalid.
//!
//! Here's an example using a smaller list:
//!
//! Suppose we instead only had a circular list containing five elements, `0, 1, 2, 3, 4`, and were given input lengths of `3, 4, 1, 5`.
//!
//! *   The list begins as `[0] 1 2 3 4` (where square brackets indicate the _current position_).
//! *   The first length, `3`, selects `([0] 1 2) 3 4` (where parentheses indicate the sublist to be reversed).
//! *   After reversing that section (`0 1 2` into `2 1 0`), we get `([2] 1 0) 3 4`.
//! *   Then, the _current position_ moves forward by the _length_, `3`, plus the _skip size_, 0: `2 1 0 [3] 4`. Finally, the _skip size_ increases to `1`.
//!
//! *   The second length, `4`, selects a section which wraps: `2 1) 0 ([3] 4`.
//! *   The sublist `3 4 2 1` is reversed to form `1 2 4 3`: `4 3) 0 ([1] 2`.
//! *   The _current position_ moves forward by the _length_ plus the _skip size_, a total of `5`, causing it not to move because it wraps around: `4 3 0 [1] 2`. The _skip size_ increases to `2`.
//!
//! *   The third length, `1`, selects a sublist of a single element, and so reversing it has no effect.
//! *   The _current position_ moves forward by the _length_ (`1`) plus the _skip size_ (`2`): `4 [3] 0 1 2`. The _skip size_ increases to `3`.
//!
//! *   The fourth length, `5`, selects every element starting with the second: `4) ([3] 0 1 2`. Reversing this sublist (`3 0 1 2 4` into `4 2 1 0 3`) produces: `3) ([4] 2 1 0`.
//! *   Finally, the _current position_ moves forward by `8`: `3 4 2 1 [0]`. The _skip size_ increases to `4`.
//!
//! In this example, the first two numbers in the list end up being `3` and `4`; to check the process, you can multiply them together to produce `12`.
//!
//! However, you should instead use the standard list size of `256` (with values `0` to `255`) and the sequence of _lengths_ in your puzzle input. Once this process is complete, _what is the result of multiplying the first two numbers in the list_?
//!
//! ## Part Two
//!
//! The logic you've constructed forms a single _round_ of the _Knot Hash_ algorithm; running the full thing requires many of these rounds. Some input and output processing is also required.
//!
//! First, from now on, your input should be taken not as a list of numbers, but as a string of bytes instead. Unless otherwise specified, convert characters to bytes using their [ASCII codes](https://en.wikipedia.org/wiki/ASCII#Printable_characters). This will allow you to handle arbitrary ASCII strings, and it also ensures that your input lengths are never larger than `255`. For example, if you are given `1,2,3`, you should convert it to the ASCII codes for each character: `49,44,50,44,51`.
//!
//! Once you have determined the sequence of lengths to use, add the following lengths to the end of the sequence: `17, 31, 73, 47, 23`. For example, if you are given `1,2,3`, your final sequence of lengths should be `49,44,50,44,51,17,31,73,47,23` (the ASCII codes from the input string combined with the standard length suffix values).
//!
//! Second, instead of merely running one _round_ like you did above, run a total of `64` rounds, using the same _length_ sequence in each round. The _current position_ and _skip size_ should be preserved between rounds. For example, if the previous example was your first round, you would start your second round with the same _length_ sequence (`3, 4, 1, 5, 17, 31, 73, 47, 23`, now assuming they came from ASCII codes and include the suffix), but start with the previous round's _current position_ (`4`) and _skip size_ (`4`).
//!
//! Once the rounds are complete, you will be left with the numbers from `0` to `255` in some order, called the _sparse hash_. Your next task is to reduce these to a list of only `16` numbers called the _dense hash_. To do this, use numeric bitwise [XOR](https://en.wikipedia.org/wiki/Bitwise_operation#XOR) to combine each consecutive block of `16` numbers in the sparse hash (there are `16` such blocks in a list of `256` numbers). So, the first element in the dense hash is the first sixteen elements of the sparse hash XOR'd together, the second element in the dense hash is the second sixteen elements of the sparse hash XOR'd together, etc.
//!
//! For example, if the first sixteen elements of your sparse hash are as shown below, and the XOR operator is `^`, you would calculate the first output number like this:
//!
//!```text
//! 65 ^ 27 ^ 9 ^ 1 ^ 4 ^ 3 ^ 40 ^ 50 ^ 91 ^ 7 ^ 6 ^ 0 ^ 2 ^ 5 ^ 68 ^ 22 = 64
//!```
//!
//! Perform this operation on each of the sixteen blocks of sixteen numbers in your sparse hash to determine the sixteen numbers in your dense hash.
//!
//! Finally, the standard way to represent a Knot Hash is as a single [hexadecimal](https://en.wikipedia.org/wiki/Hexadecimal) string; the final output is the dense hash in hexadecimal notation. Because each number in your dense hash will be between `0` and `255` (inclusive), always represent each number as two hexadecimal digits (including a leading zero as necessary). So, if your first three numbers are `64, 7, 255`, they correspond to the hexadecimal numbers `40, 07, ff`, and so the first six characters of the hash would be `4007ff`. Because every Knot Hash is sixteen such numbers, the hexadecimal representation is always `32` hexadecimal digits (`0`-`f`) long.
//!
//! Here are some example hashes:
//!
//! *   The empty string becomes `a2582a3a0e66e6e86e3812dcb672a272`.
//! *   `AoC 2017` becomes `33efeb34ea91902bb2f59c9920caa6cd`.
//! *   `1,2,3` becomes `3efbe78a8d82f29979031a4aa0b16a9d`.
//! *   `1,2,4` becomes `63960835bcdc130f0b66d7ff4f6a5a8e`.
//!
//! Treating your puzzle input as a string of ASCII characters, _what is the Knot Hash of your puzzle input?_ Ignore any leading or trailing whitespace you might encounter.
//!

#[derive(Debug)]
struct Puzzle {
    list: Vec<u8>,
    current_position: usize,
    skip_size: usize,
    current_length: usize,
}

#[derive(Debug)]
struct PuzzleStep1 {
    state: Puzzle,
}

#[derive(Debug)]
struct PuzzleStep2 {
    state: Puzzle,
}

#[derive(Debug)]
struct PuzzleStep3 {
    state: Puzzle,
}

impl Puzzle {
    fn from_list_size(list_size: usize) -> PuzzleStep1 {
        PuzzleStep1 {
            state: Puzzle {
                list: (0..list_size).map(|i| i as u8).collect(),
                current_position: 0,
                skip_size: 0,
                current_length: 0,
            },
        }
    }
}

impl PuzzleStep1 {
    fn steps(self, length: usize) -> PuzzleStep1 {
        let puzzle = self.reverse(length);
        let puzzle = puzzle.move_current_position();
        puzzle.increase()
    }

    fn reverse(self, length: usize) -> PuzzleStep2 {
        // Reverse the order of that length of elements in the list,
        // starting with the element at the current position.

        assert!(length <= self.state.list.len());

        let mut puzzle = PuzzleStep2 { state: self.state };

        // Store this to be used by step 2
        puzzle.state.current_length = length;

        let current_position = puzzle.state.current_position;

        let reverted_slice = {
            let mut tmp: Vec<u8> = puzzle
                .state
                .list
                .iter()
                .cycle()
                .skip(current_position)
                .take(length as usize)
                .cloned()
                .collect();
            tmp.reverse();
            tmp
        };

        // Replace start of sequence up to list's upper bound
        puzzle
            .state
            .list
            .iter_mut()
            .skip(current_position)
            .take(length)
            .zip(reverted_slice.iter())
            .for_each(|(to_replace, to_replace_with)| *to_replace = *to_replace_with);

        // Replace end of sequence (if wrap occurred)
        let list_size = puzzle.state.list.len();
        let overflow_happens = current_position + length > list_size;

        if overflow_happens {
            let nb_overflowed = current_position + length - list_size;
            let overflow_starts = length - nb_overflowed;
            puzzle
                .state
                .list
                .iter_mut()
                .zip(
                    reverted_slice
                        .iter()
                        .skip(overflow_starts)
                        .take(nb_overflowed),
                )
                .for_each(|(to_replace, to_replace_with)| *to_replace = *to_replace_with);
        }

        puzzle
    }
}

impl PuzzleStep2 {
    fn move_current_position(self) -> PuzzleStep3 {
        // Move the current position forward by that length plus the skip size.
        let new_position = (self.state.current_position + self.state.current_length
            + self.state.skip_size) % self.state.list.len();

        let mut puzzle = PuzzleStep3 { state: self.state };
        puzzle.state.current_position = new_position;

        puzzle
    }
}

impl PuzzleStep3 {
    fn increase(self) -> PuzzleStep1 {
        // Increase the skip size by one.
        let mut puzzle = PuzzleStep1 { state: self.state };
        puzzle.state.skip_size += 1;

        puzzle
    }
}

fn puzzle_steps(puzzle: PuzzleStep1, lengths: &[usize]) -> PuzzleStep1 {
    let mut puzzle_state = puzzle.state;
    for length in lengths.iter() {
        let puzzle = PuzzleStep1 {
            state: puzzle_state,
        };
        let puzzle = puzzle.steps(*length);
        // Store state in variable outside the loop since `puzzle` will be droped
        puzzle_state = puzzle.state;
    }

    PuzzleStep1 {
        state: puzzle_state,
    }
}

fn create_and_advance_puzzle(lengths: &[usize], list_size: usize, nb_run: usize) -> PuzzleStep1 {
    let mut puzzle = Puzzle::from_list_size(list_size);

    for _i in 0..nb_run {
        puzzle = puzzle_steps(puzzle, lengths);
    }

    puzzle
}

pub fn aoc_day10_slice(lengths: &[usize], list_size: usize, nb_run: usize) -> usize {
    let puzzle = create_and_advance_puzzle(lengths, list_size, nb_run);
    (puzzle.state.list[0] as usize) * (puzzle.state.list[1] as usize)
}

pub mod part_1 {
    use super::aoc_day10_slice;

    pub fn aoc_day10(input: &str, input_list_size: usize) -> usize {
        let lengths: Vec<usize> = input
            .trim()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        let nb_run = 1;
        aoc_day10_slice(lengths.as_slice(), input_list_size, nb_run)
    }
}

pub mod part_2 {
    use super::create_and_advance_puzzle;

    pub fn aoc_day10(input: &str, input_list_size: usize) -> String {
        let mut lengths: Vec<usize> = input.trim().chars().map(|v| v as usize).collect();

        lengths.extend([17, 31, 73, 47, 23].iter());

        let nb_run = 64;
        let puzzle = create_and_advance_puzzle(lengths.as_slice(), input_list_size, nb_run);

        puzzle.state.list
            .chunks(16)
            .map(|chunk| chunk.iter().fold(0, |acc, v| acc ^ v))  // dense_hash
            .map(|v| format!("{:02x}", v))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day10 {
            const PUZZLE_INPUT: &'static str = include_str!("../input");

            mod part_1 {
                use ::*;
                use super::PUZZLE_INPUT;

                #[test]
                fn example_01_steps_manual() {
                    let puzzle = Puzzle::from_list_size(5);

                    // Initial conditions
                    assert_eq!([0, 1, 2, 3, 4], puzzle.state.list.as_slice());
                    assert_eq!(0, puzzle.state.current_position);
                    assert_eq!(0, puzzle.state.skip_size);

                    // First length
                    let puzzle = puzzle.reverse(3);
                    assert_eq!([2, 1, 0, 3, 4], puzzle.state.list.as_slice());
                    assert_eq!(0, puzzle.state.current_position);
                    assert_eq!(0, puzzle.state.skip_size);

                    let puzzle = puzzle.move_current_position();
                    assert_eq!([2, 1, 0, 3, 4], puzzle.state.list.as_slice());
                    assert_eq!(3, puzzle.state.current_position);
                    assert_eq!(0, puzzle.state.skip_size);

                    let puzzle = puzzle.increase();
                    assert_eq!([2, 1, 0, 3, 4], puzzle.state.list.as_slice());
                    assert_eq!(3, puzzle.state.current_position);
                    assert_eq!(1, puzzle.state.skip_size);

                    // Second length
                    let puzzle = puzzle.reverse(4);
                    assert_eq!([4, 3, 0, 1, 2], puzzle.state.list.as_slice());
                    assert_eq!(3, puzzle.state.current_position);
                    assert_eq!(1, puzzle.state.skip_size);

                    let puzzle = puzzle.move_current_position();
                    assert_eq!([4, 3, 0, 1, 2], puzzle.state.list.as_slice());
                    assert_eq!(3, puzzle.state.current_position);
                    assert_eq!(1, puzzle.state.skip_size);

                    let puzzle = puzzle.increase();
                    assert_eq!([4, 3, 0, 1, 2], puzzle.state.list.as_slice());
                    assert_eq!(3, puzzle.state.current_position);
                    assert_eq!(2, puzzle.state.skip_size);

                    // Third length
                    let puzzle = puzzle.reverse(1);
                    assert_eq!([4, 3, 0, 1, 2], puzzle.state.list.as_slice());
                    assert_eq!(3, puzzle.state.current_position);
                    assert_eq!(2, puzzle.state.skip_size);

                    let puzzle = puzzle.move_current_position();
                    assert_eq!([4, 3, 0, 1, 2], puzzle.state.list.as_slice());
                    assert_eq!(1, puzzle.state.current_position);
                    assert_eq!(2, puzzle.state.skip_size);

                    let puzzle = puzzle.increase();
                    assert_eq!([4, 3, 0, 1, 2], puzzle.state.list.as_slice());
                    assert_eq!(1, puzzle.state.current_position);
                    assert_eq!(3, puzzle.state.skip_size);

                    // Fourth length
                    let puzzle = puzzle.reverse(5);
                    assert_eq!([3, 4, 2, 1, 0], puzzle.state.list.as_slice());
                    assert_eq!(1, puzzle.state.current_position);
                    assert_eq!(3, puzzle.state.skip_size);

                    let puzzle = puzzle.move_current_position();
                    assert_eq!([3, 4, 2, 1, 0], puzzle.state.list.as_slice());
                    assert_eq!(4, puzzle.state.current_position);
                    assert_eq!(3, puzzle.state.skip_size);

                    let puzzle = puzzle.increase();
                    assert_eq!([3, 4, 2, 1, 0], puzzle.state.list.as_slice());
                    assert_eq!(4, puzzle.state.current_position);
                    assert_eq!(4, puzzle.state.skip_size);
                }

                #[test]
                fn example_01_steps_fn() {
                    let puzzle = Puzzle::from_list_size(5);

                    // Initial conditions
                    assert_eq!([0, 1, 2, 3, 4], puzzle.state.list.as_slice());
                    assert_eq!(0, puzzle.state.current_position);
                    assert_eq!(0, puzzle.state.skip_size);

                    // let lengths = [3, 4, 1, 5];
                    let puzzle = puzzle.steps(3);
                    assert_eq!([2, 1, 0, 3, 4], puzzle.state.list.as_slice());
                    assert_eq!(3, puzzle.state.current_position);
                    assert_eq!(1, puzzle.state.skip_size);

                    let puzzle = puzzle.steps(4);
                    assert_eq!([4, 3, 0, 1, 2], puzzle.state.list.as_slice());
                    assert_eq!(3, puzzle.state.current_position);
                    assert_eq!(2, puzzle.state.skip_size);

                    let puzzle = puzzle.steps(1);
                    assert_eq!([4, 3, 0, 1, 2], puzzle.state.list.as_slice());
                    assert_eq!(1, puzzle.state.current_position);
                    assert_eq!(3, puzzle.state.skip_size);

                    let puzzle = puzzle.steps(5);
                    assert_eq!([3, 4, 2, 1, 0], puzzle.state.list.as_slice());
                    assert_eq!(4, puzzle.state.current_position);
                    assert_eq!(4, puzzle.state.skip_size);
                }

                #[test]
                fn example_01_slice() {
                    let expected = 12;
                    let input = [3, 4, 1, 5];
                    let input_list_size = 5;
                    let nb_run = 1;
                    let to_check = aoc_day10_slice(&input, input_list_size, nb_run);

                    assert_eq!(expected, to_check);
                }

                #[test]
                fn example_01_solution() {
                    let expected = 12;
                    let input = "3,4,1,5";
                    let input_list_size = 5;
                    let to_check = part_1::aoc_day10(input, input_list_size);

                    assert_eq!(expected, to_check);
                }

                #[test]
                fn solution() {
                    let expected = 826;
                    let input_list_size = 256;
                    let to_check = part_1::aoc_day10(PUZZLE_INPUT, input_list_size);

                    assert_eq!(expected, to_check);
                }
            }

            mod part_2 {
                use ::*;
                use super::PUZZLE_INPUT;

                #[test]
                fn example_01_empty_string() {
                    let expected = "a2582a3a0e66e6e86e3812dcb672a272";
                    let input = "";
                    let input_list_size = 256;
                    let to_check = part_2::aoc_day10(input, input_list_size);

                    assert_eq!(expected, to_check);
                }

                #[test]
                fn example_02_aoc_2017() {
                    let expected = "33efeb34ea91902bb2f59c9920caa6cd";
                    let input = "AoC 2017";
                    let input_list_size = 256;
                    let to_check = part_2::aoc_day10(input, input_list_size);

                    assert_eq!(expected, to_check);
                }

                #[test]
                fn example_03_123() {
                    let expected = "3efbe78a8d82f29979031a4aa0b16a9d";
                    let input = "1,2,3";
                    let input_list_size = 256;
                    let to_check = part_2::aoc_day10(input, input_list_size);

                    assert_eq!(expected, to_check);
                }

                #[test]
                fn example_04_124() {
                    let expected = "63960835bcdc130f0b66d7ff4f6a5a8e";
                    let input = "1,2,4";
                    let input_list_size = 256;
                    let to_check = part_2::aoc_day10(input, input_list_size);

                    assert_eq!(expected, to_check);
                }

                #[test]
                fn solution() {
                    let expected = "d067d3f14d07e09c2e7308c3926605c4";
                    let input_list_size = 256;
                    let to_check = part_2::aoc_day10(PUZZLE_INPUT, input_list_size);

                    assert_eq!(expected, to_check);
                }
            }
        }
    }
}
