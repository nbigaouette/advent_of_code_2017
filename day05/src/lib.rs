//! # Day 5: A Maze of Twisty Trampolines, All Alike
//!
//! ## Part One
//!
//! An urgent interrupt arrives from the CPU: it's trapped in a maze of jump instructions, and it would like assistance from any programs with spare cycles to help find the exit.
//!
//! The message includes a list of the offsets for each jump. Jumps are relative: `-1` moves to the previous instruction, and `2` skips the next one. Start at the first instruction in the list. The goal is to follow the jumps until one leads _outside_ the list.
//!
//! In addition, these instructions are a little strange; after each jump, the offset of that instruction increases by `1`. So, if you come across an offset of `3`, you would move three instructions forward, but change it to a `4` for the next time it is encountered.
//!
//! For example, consider the following list of jump offsets:
//!
//!```text
//!     0
//!     3
//!     0
//!     1
//!     -3
//!```
//!
//! Positive jumps ("forward") move downward; negative jumps move upward. For legibility in this example, these offset values will be written all on one line, with the current instruction marked in parentheses. The following steps would be taken before an exit is found:
//!
//! *   `(0) 3  0  1  -3 ` \- _before_ we have taken any steps.
//! *   `(1) 3  0  1  -3 ` \- jump with offset `0` (that is, don't jump at all). Fortunately, the instruction is then incremented to `1`.
//! *   ` 2 (3) 0  1  -3 ` \- step forward because of the instruction we just modified. The first instruction is incremented again, now to `2`.
//! *   ` 2  4  0  1 (-3)` \- jump all the way to the end; leave a `4` behind.
//! *   ` 2 (4) 0  1  -2 ` \- go back to where we just were; increment `-3` to `-2`.
//! *   ` 2  5  0  1  -2 ` \- jump `4` steps forward, escaping the maze.
//!
//! In this example, the exit is reached in `5` steps.
//!
//! _How many steps_ does it take to reach the exit?
//!
//! ## Part Two
//!
//! Now, the jumps are even stranger: after each jump, if the offset was _three or more_, instead _decrease_ it by `1`. Otherwise, increase it by `1` as before.
//!
//! Using this rule with the above example, the process now takes `10` steps, and the offset values after finding the exit are left as `2 3 2 3 -1`.
//!
//! _How many steps_ does it now take to reach the exit?

pub struct Instructions<T>
where
    T: Fn(i32) -> bool,
{
    set: Vec<i32>,
    ptr: i32,
    check: T,
}

impl<T> Instructions<T>
where
    T: Fn(i32) -> bool,
{
    pub fn new(instructions: &[i32], check: T) -> Instructions<T> {
        Instructions {
            set: instructions.to_vec(),
            ptr: 0,
            check,
        }
    }
    pub fn set(&self) -> &[i32] {
        &self.set
    }
}

impl<T> Iterator for Instructions<T>
where
    T: Fn(i32) -> bool,
{
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        // Scope the borrow on self (for `instruction`)
        {
            // Fetch the instruction
            let instruction = &mut self.set[self.ptr as usize];

            // Jump pointer
            self.ptr += *instruction;

            // Increment the instruction
            *instruction += if (self.check)(*instruction) { -1 } else { 1 };
        }

        if 0 <= self.ptr && self.ptr < self.set.len() as i32 {
            Some((self.ptr, self.set[self.ptr as usize]))
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|n| n.trim().parse().unwrap()).collect()
}

fn aoc_day05_generic(instructions: &str, check: fn(i32) -> bool) -> u32 {
    let instructions = parse_input(instructions);
    let instruction_set = Instructions::new(&instructions, check);
    instruction_set.count() as u32 + 1
}

pub mod part_1 {
    use super::aoc_day05_generic;

    pub fn aoc_day05(instructions: &str) -> u32 {
        let check = |_| false;
        aoc_day05_generic(instructions, check)
    }
}

pub mod part_2 {
    use super::aoc_day05_generic;

    pub fn aoc_day05(instructions: &str) -> u32 {
        let check = |instruction| instruction >= 3;
        aoc_day05_generic(instructions, check)
    }
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day05 {
            const PUZZLE_INPUT: &'static str = include_str!("../input");

            mod part_1 {
                use ::*;
                use super::PUZZLE_INPUT;

                #[test]
                fn iterator_steps() {
                    let check = |_| false;
                    let mut instruction_set = Instructions::new(&[0, 3, 0, 1, -3], check);

                    // (0) 3  0  1  -3
                    assert_eq!(&[0, 3, 0, 1, -3], instruction_set.set());

                    // (1) 3  0  1  -3
                    let ptr_inst = instruction_set.next();
                    assert_eq!(&[1, 3, 0, 1, -3], instruction_set.set());
                    assert_eq!(Some((0, 1)), ptr_inst);
                    assert_eq!(
                        ptr_inst.unwrap().1,
                        instruction_set.set()[ptr_inst.unwrap().0 as usize]
                    );

                    //  2 (3) 0  1  -3
                    let ptr_inst = instruction_set.next();
                    assert_eq!(&[2, 3, 0, 1, -3], instruction_set.set());
                    assert_eq!(Some((1, 3)), ptr_inst);
                    assert_eq!(
                        ptr_inst.unwrap().1,
                        instruction_set.set()[ptr_inst.unwrap().0 as usize]
                    );

                    //  2  4  0  1 (-3)
                    let ptr_inst = instruction_set.next();
                    assert_eq!(&[2, 4, 0, 1, -3], instruction_set.set());
                    assert_eq!(Some((4, -3)), ptr_inst);
                    assert_eq!(
                        ptr_inst.unwrap().1,
                        instruction_set.set()[ptr_inst.unwrap().0 as usize]
                    );

                    //  2 (4) 0  1  -2
                    let ptr_inst = instruction_set.next();
                    assert_eq!(&[2, 4, 0, 1, -2], instruction_set.set());
                    assert_eq!(Some((1, 4)), ptr_inst);
                    assert_eq!(
                        ptr_inst.unwrap().1,
                        instruction_set.set()[ptr_inst.unwrap().0 as usize]
                    );

                    //  2  5  0  1  -2
                    let ptr_inst = instruction_set.next();
                    assert_eq!(None, ptr_inst);
                    assert_eq!(&[2, 5, 0, 1, -2], instruction_set.set());
                }

                #[test]
                fn example_01() {
                    let input = "0
                             3
                             0
                             1
                             -3";
                    let expected = 5;
                    let to_check = part_1::aoc_day05(&input);

                    assert_eq!(expected, to_check);
                }

                #[test]
                fn solution() {
                    let expected = 358309;
                    let to_check = part_1::aoc_day05(PUZZLE_INPUT);

                    assert_eq!(expected, to_check);
                }
            }

            mod part_2 {
                use ::*;
                use super::PUZZLE_INPUT;

                #[test]
                fn example_01_parse_input() {
                    let input = "0
                             3
                             0
                             1
                             -3";
                    let expected = [0, 3, 0, 1, -3];
                    let to_check = parse_input(&input);

                    assert_eq!(to_check, expected);
                }

                #[test]
                fn example_01_steps() {
                    let check = |instruction| instruction >= 3;
                    let mut instructions = Instructions::new(&[0, 3, 0, 1, -3], check);

                    let (ptr, val) = instructions.next().unwrap();
                    assert_eq!(0, ptr);
                    assert_eq!(1, val);
                    assert_eq!(&[1, 3, 0, 1, -3], instructions.set());

                    let (ptr, val) = instructions.next().unwrap();
                    assert_eq!(1, ptr);
                    assert_eq!(3, val);
                    assert_eq!(&[2, 3, 0, 1, -3], instructions.set());

                    let (ptr, val) = instructions.next().unwrap();
                    assert_eq!(4, ptr);
                    assert_eq!(-3, val);
                    assert_eq!(&[2, 2, 0, 1, -3], instructions.set());

                    let (ptr, val) = instructions.next().unwrap();
                    assert_eq!(1, ptr);
                    assert_eq!(2, val);
                    assert_eq!(&[2, 2, 0, 1, -2], instructions.set());

                    let (ptr, val) = instructions.next().unwrap();
                    assert_eq!(3, ptr);
                    assert_eq!(1, val);
                    assert_eq!(&[2, 3, 0, 1, -2], instructions.set());

                    let (ptr, val) = instructions.next().unwrap();
                    assert_eq!(4, ptr);
                    assert_eq!(-2, val);
                    assert_eq!(&[2, 3, 0, 2, -2], instructions.set());

                    let (ptr, val) = instructions.next().unwrap();
                    assert_eq!(2, ptr);
                    assert_eq!(0, val);
                    assert_eq!(&[2, 3, 0, 2, -1], instructions.set());

                    let (ptr, val) = instructions.next().unwrap();
                    assert_eq!(2, ptr);
                    assert_eq!(1, val);
                    assert_eq!(&[2, 3, 1, 2, -1], instructions.set());

                    let (ptr, val) = instructions.next().unwrap();
                    assert_eq!(3, ptr);
                    assert_eq!(2, val);
                    assert_eq!(&[2, 3, 2, 2, -1], instructions.set());

                    let end = instructions.next();
                    assert!(end.is_none());
                    assert_eq!(&[2, 3, 2, 3, -1], instructions.set());
                }

                #[test]
                fn example_01() {
                    let input = "0
                             3
                             0
                             1
                             -3";
                    let expected = 10;
                    let to_check = part_2::aoc_day05(&input);

                    assert_eq!(expected, to_check);
                }

                #[test]
                fn solution() {
                    let expected = 28178177;
                    let to_check = part_2::aoc_day05(PUZZLE_INPUT);

                    assert_eq!(expected, to_check);
                }
            }
        }
    }
}
