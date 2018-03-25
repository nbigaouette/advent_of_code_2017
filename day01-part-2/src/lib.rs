pub fn aoc_day01_part_two(input: &str) -> u32 {
    input
        .chars()
        .cycle()
        .skip(input.len() / 2)
        .zip(input.chars())
        .fold(0, |acc, (u, l)| {
            if u == l {
                acc + l.to_digit(10).unwrap()
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day01_part_two {
            use ::*;

            #[test]
            fn example_01_1212() {
                let input = "1212";
                let expected = 6;
                let to_check = aoc_day01_part_two(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_02_1221() {
                let input = "1221";
                let expected = 0;
                let to_check = aoc_day01_part_two(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_03_123425() {
                let input = "123425";
                let expected = 4;
                let to_check = aoc_day01_part_two(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_04_123123() {
                let input = "123123";
                let expected = 12;
                let to_check = aoc_day01_part_two(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_05_12131415() {
                let input = "12131415";
                let expected = 4;
                let to_check = aoc_day01_part_two(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution() {
                const PUZZLE_INPUT: &'static str = include_str!("../day01_input.txt");
                let expected = 1060;
                let to_check = aoc_day01_part_two(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
