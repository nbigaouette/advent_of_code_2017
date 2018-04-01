use std::collections::HashMap;

fn is_passphrase_valid(passphrase: &str) -> bool {
    let mut passphrase_word_count = HashMap::new();

    passphrase.split_whitespace().for_each(|word| {
        let word_count = passphrase_word_count.entry(word).or_insert(0);
        *word_count += 1;
    });

    let max_count = passphrase_word_count
        .iter()
        .fold(0, |max_count, (_word, count)| max_count.max(*count));

    max_count == 1
}

pub fn aoc_day04_part_1(passphrase_list: &str) -> u32 {
    passphrase_list
        .lines()
        .map(|passphrase| is_passphrase_valid(passphrase))
        .fold(0, |acc, is_valid| if is_valid { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day04_part_1 {
            use ::*;

            #[test]
            fn example_01_aa_bb_cc_dd_ee() {
                let input = "aa bb cc dd ee";
                let expected = true;
                let to_check = is_passphrase_valid(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_02_aa_bb_cc_dd_aa() {
                let input = "aa bb cc dd aa";
                let expected = false;
                let to_check = is_passphrase_valid(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_03_aa_bb_cc_dd_aaa() {
                let input = "aa bb cc dd aaa";
                let expected = true;
                let to_check = is_passphrase_valid(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution() {
                const PUZZLE_INPUT: &'static str = include_str!("../input.txt");
                let expected = 383;
                let to_check = aoc_day04_part_1(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
