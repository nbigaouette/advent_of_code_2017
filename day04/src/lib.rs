//! # Day 4: High-Entropy Passphrases
//!
//! ## Part One
//!
//! A new system policy has been put in place that requires all accounts to use a _passphrase_ instead of simply a pass_word_. A passphrase consists of a series of words (lowercase letters) separated by spaces.
//!
//! To ensure security, a valid passphrase must contain no duplicate words.
//!
//! For example:
//!
//! *   `aa bb cc dd ee` is valid.
//! *   `aa bb cc dd aa` is not valid - the word `aa` appears more than once.
//! *   `aa bb cc dd aaa` is valid - `aa` and `aaa` count as different words.
//!
//! The system's full passphrase list is available as your puzzle input. _How many passphrases are valid?_
//!
//! ## Part Two
//!
//! For added security, yet another system policy has been put in place. Now, a valid passphrase must contain no two words that are anagrams of each other - that is, a passphrase is invalid if any word's letters can be rearranged to form any other word in the passphrase.
//!
//! For example:
//!
//! *   `abcde fghij` is a valid passphrase.
//! *   `abcde xyz ecdab` is not valid - the letters from the third word can be rearranged to form the first word.
//! *   `a ab abc abd abf abj` is a valid passphrase, because _all_ letters need to be used when forming another word.
//! *   `iiii oiii ooii oooi oooo` is valid.
//! *   `oiii ioii iioi iiio` is not valid - any of these words can be rearranged to form any other word.
//!
//! Under this new system policy, _how many passphrases are valid?_

use std::collections::HashMap;

fn is_passphrase_valid_part_1(passphrase: &str) -> bool {
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

fn is_passphrase_valid_part_2(passphrase: &str) -> bool {
    let mut passphrase_word_count = HashMap::new();

    passphrase.split_whitespace().for_each(|word| {
        let mut word_letters = word.chars().collect::<Vec<_>>();
        word_letters.sort();

        let word_letters_count = passphrase_word_count.entry(word_letters).or_insert(0);
        *word_letters_count += 1;
    });

    let max_count = passphrase_word_count
        .iter()
        .fold(0, |max_count, (_word, count)| max_count.max(*count));

    max_count == 1
}

pub fn aoc_day04_part_1(passphrase_list: &str) -> u32 {
    passphrase_list
        .lines()
        .map(|passphrase| is_passphrase_valid_part_1(passphrase))
        .fold(0, |acc, is_valid| if is_valid { acc + 1 } else { acc })
}

pub fn aoc_day04_part_2(passphrase_list: &str) -> u32 {
    passphrase_list
        .lines()
        .map(|passphrase| is_passphrase_valid_part_2(passphrase))
        .fold(0, |acc, is_valid| if is_valid { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day04 {
            use ::*;

            const PUZZLE_INPUT: &'static str = include_str!("../input");

            #[test]
            fn part_1_example_01_aa_bb_cc_dd_ee() {
                let input = "aa bb cc dd ee";
                let expected = true;
                let to_check = is_passphrase_valid_part_1(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_1_example_02_aa_bb_cc_dd_aa() {
                let input = "aa bb cc dd aa";
                let expected = false;
                let to_check = is_passphrase_valid_part_1(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_1_example_03_aa_bb_cc_dd_aaa() {
                let input = "aa bb cc dd aaa";
                let expected = true;
                let to_check = is_passphrase_valid_part_1(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_example_01_abcde_fghij() {
                let input = "abcde fghij";
                let expected = true;
                let to_check = is_passphrase_valid_part_2(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_example_02_abcde_xyz_ecdaba() {
                let input = "abcde xyz ecdab";
                let expected = false;
                let to_check = is_passphrase_valid_part_2(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_example_03_a_ab_abc_abd_abf_abj() {
                let input = "a ab abc abd abf abj";
                let expected = true;
                let to_check = is_passphrase_valid_part_2(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_example_04_iiii_oiii_ooii_oooi_oooo() {
                let input = "iiii oiii ooii oooi oooo";
                let expected = true;
                let to_check = is_passphrase_valid_part_2(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_example_05_oiii_ioii_iioi_iiio() {
                let input = "oiii ioii iioi iiio";
                let expected = false;
                let to_check = is_passphrase_valid_part_2(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_1_solution() {
                let expected = 383;
                let to_check = aoc_day04_part_1(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_solution() {
                let expected = 265;
                let to_check = aoc_day04_part_2(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
