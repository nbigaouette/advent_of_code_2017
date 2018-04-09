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

pub struct Instructions {
    set: Vec<i32>,
    ptr: i32,
}

impl Instructions {
    pub fn new(instructions: &[i32]) -> Instructions {
        Instructions {
            set: instructions.to_vec(),
            ptr: 0,
        }
    }
    pub fn set(&self) -> &[i32] {
        &self.set
    }
}

impl Iterator for Instructions {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        // Scope the borrow on self (for `instruction`)
        {
            // Fetch the instruction
            let instruction = &mut self.set[self.ptr as usize];

            // Jump pointer
            self.ptr += *instruction;

            // Increment the instruction
            *instruction += 1;
        }

        if 0 <= self.ptr && self.ptr < self.set.len() as i32 {
            Some((self.ptr, self.set[self.ptr as usize]))
        } else {
            None
        }
    }
}

pub fn aoc_day05_part_1(instructions: &str) -> u32 {
    let instructions: Vec<i32> = instructions
        .lines()
        .map(|n| n.trim().parse().unwrap())
        .collect();
    let instruction_set = Instructions::new(&instructions);
    instruction_set.count() as u32 + 1
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day05_part_1 {
            use ::*;

            #[test]
            fn iterator_steps() {
                let mut instruction_set = Instructions::new(&[0, 3, 0, 1, -3]);

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
                let to_check = aoc_day05_part_1(&input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution() {
                const PUZZLE_INPUT: &'static str = include_str!("../input.txt");
                let expected = 358309;
                let to_check = aoc_day05_part_1(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
