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
