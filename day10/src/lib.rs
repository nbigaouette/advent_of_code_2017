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

pub fn aoc_day10_part_2(input: &str, input_list_size: usize) -> String {
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

pub fn aoc_day10(input: &str, input_list_size: usize) -> usize {
    let lengths: Vec<usize> = input
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect();
    let nb_run = 1;
    aoc_day10_slice(lengths.as_slice(), input_list_size, nb_run)
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day10 {
            use ::*;

            const PUZZLE_INPUT: &'static str = include_str!("../input");

            #[test]
            fn part_1_example_01_steps_manual() {
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
            fn part_1_example_01_steps_fn() {
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
            fn part_1_example_01_slice() {
                let expected = 12;
                let input = [3, 4, 1, 5];
                let input_list_size = 5;
                let nb_run = 1;
                let to_check = aoc_day10_slice(&input, input_list_size, nb_run);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_1_example_01_solution() {
                let expected = 12;
                let input = "3,4,1,5";
                let input_list_size = 5;
                let to_check = aoc_day10(input, input_list_size);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_example_01_empty_string() {
                let expected = "a2582a3a0e66e6e86e3812dcb672a272";
                let input = "";
                let input_list_size = 256;
                let to_check = aoc_day10_part_2(input, input_list_size);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_example_02_aoc_2017() {
                let expected = "33efeb34ea91902bb2f59c9920caa6cd";
                let input = "AoC 2017";
                let input_list_size = 256;
                let to_check = aoc_day10_part_2(input, input_list_size);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_example_03_123() {
                let expected = "3efbe78a8d82f29979031a4aa0b16a9d";
                let input = "1,2,3";
                let input_list_size = 256;
                let to_check = aoc_day10_part_2(input, input_list_size);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_example_04_124() {
                let expected = "63960835bcdc130f0b66d7ff4f6a5a8e";
                let input = "1,2,4";
                let input_list_size = 256;
                let to_check = aoc_day10_part_2(input, input_list_size);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_1_solution() {
                let expected = 826;
                let input_list_size = 256;
                let to_check = aoc_day10(PUZZLE_INPUT, input_list_size);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_solution() {
                let expected = "d067d3f14d07e09c2e7308c3926605c4";
                let input_list_size = 256;
                let to_check = aoc_day10_part_2(PUZZLE_INPUT, input_list_size);

                assert_eq!(expected, to_check);
            }
        }
    }
}