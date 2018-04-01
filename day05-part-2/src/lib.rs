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
            *instruction += if *instruction >= 3 { -1 } else { 1 };
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

pub fn aoc_day05_part_2(instructions: &str) -> u32 {
    let instructions = parse_input(instructions);
    let instruction_set = Instructions::new(&instructions);
    instruction_set.count() as u32 + 1
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day05_part_2 {
            use ::*;

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
                let mut instructions = Instructions::new(&[0, 3, 0, 1, -3]);

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
                let to_check = aoc_day05_part_2(&input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution() {
                const PUZZLE_INPUT: &'static str = include_str!("../input.txt");
                let expected = 28178177;
                let to_check = aoc_day05_part_2(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
