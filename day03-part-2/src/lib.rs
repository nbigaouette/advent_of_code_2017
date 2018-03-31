pub fn aoc_day03_part_2(n: u32) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day03_part_2 {
            use ::*;

            #[test]
            fn solution() {
                let puzzle_input = 325489;
                let expected = 0;
                let to_check = aoc_day03_part_2(puzzle_input);

                assert_eq!(expected, to_check);
            }
        }
    }
}
