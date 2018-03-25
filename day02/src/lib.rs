pub fn aoc_day02(input: &str) -> u32 {
    // For each row, determine the difference between the largest value and the smallest value;
    // the checksum is the sum of all of these differences.

    input
        .lines()
        .map(|line| {
            let min_max = line.trim().split_whitespace().fold(
                (u32::max_value(), u32::min_value()),
                |mut min_max, word| {
                    let number = word.parse::<u32>().unwrap();
                    if number < min_max.0 {
                        min_max.0 = number;
                    }
                    if number > min_max.1 {
                        min_max.1 = number;
                    }
                    min_max
                },
            );
            min_max.1 - min_max.0
        })
        .fold(0, |acc, diff| acc + diff)
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day02 {
            use ::*;

            #[test]
            fn example_01_1122() {
                let input = "5 1 9 5
                             7 5 3
                             2 4 6 8";
                let expected = 18;
                let to_check = aoc_day02(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution() {
                const PUZZLE_INPUT: &'static str = include_str!("../day02_input.txt");
                let expected = 47623;
                let to_check = aoc_day02(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
