pub fn aoc_day02_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let it = line.trim().split_whitespace();
            let mut large_small = (0, 0);
            for (i, left_str) in it.clone().enumerate() {
                let left: u32 = left_str.parse().unwrap();
                for right_str in it.clone().skip(i + 1) {
                    let right: u32 = right_str.parse().unwrap();
                    if left % right == 0 {
                        large_small.0 = left;
                        large_small.1 = right;
                    } else if right % left == 0 {
                        large_small.0 = right;
                        large_small.1 = left;
                    }
                }
            }
            large_small.0 / large_small.1
        })
        .fold(0, |acc, diff| acc + diff)
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day02_part_2 {
            use ::*;

            #[test]
            fn example_01() {
                let input = "5 9 2 8
                             9 4 7 3
                             3 8 6 5";
                let expected = 9;
                let to_check = aoc_day02_part_2(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution() {
                const PUZZLE_INPUT: &'static str = include_str!("../input.txt");
                let expected = 312;
                let to_check = aoc_day02_part_2(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
