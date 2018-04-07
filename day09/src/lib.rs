pub fn aoc_day09(input: &str) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day09 {
            use ::*;

            const EXAMPLE_O1: [(&str, u32); 8] = [
                ("{}", 1),
                ("{{{}}}", 6),
                ("{{},{}}", 5),
                ("{{{},{},{{}}}}", 16),
                ("{<a>,<a>,<a>,<a>}", 1),
                ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
                ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
                ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
            ];

            // const PUZZLE_INPUT: &'static str = include_str!("../input");

            #[test]
            fn example_01() {
                for &(input, expected) in EXAMPLE_O1.iter() {
                    let to_check = aoc_day09(input);
                    assert_eq!(expected, to_check);
                }
            }

            #[test]
            fn solution() {
                // let expected = ???;
                // let input = ???;
                // let to_check = aoc_day09(PUZZLE_INPUT);

                // assert_eq!(expected, to_check);
            }
        }
    }
}
