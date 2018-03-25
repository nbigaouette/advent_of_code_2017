pub fn aoc_day01(input: &str) -> u32 {
    input
        .chars()
        .cycle()
        .skip(1)
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
        mod day01 {
            use ::*;

            #[test]
            fn example_01_1122() {
                let input = "1122";
                let expected = 3;
                let to_check = aoc_day01(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_02_1111() {
                let input = "1111";
                let expected = 4;
                let to_check = aoc_day01(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_03_1234() {
                let input = "1234";
                let expected = 0;
                let to_check = aoc_day01(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_04_91212129() {
                let input = "91212129";
                let expected = 9;
                let to_check = aoc_day01(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution() {
                const PUZZLE_INPUT: &'static str = include_str!("../day01_input.txt");
                let expected = 1177;
                let to_check = aoc_day01(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
