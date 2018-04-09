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

fn is_passphrase_valid(passphrase: &str) -> bool {
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

pub fn aoc_day04_part_2(passphrase_list: &str) -> u32 {
    passphrase_list
        .lines()
        .map(|passphrase| is_passphrase_valid(passphrase))
        .fold(0, |acc, is_valid| if is_valid { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day04_part_2 {
            use ::*;

            #[test]
            fn example_01_abcde_fghij() {
                let input = "abcde fghij";
                let expected = true;
                let to_check = is_passphrase_valid(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_02_abcde_xyz_ecdaba() {
                let input = "abcde xyz ecdab";
                let expected = false;
                let to_check = is_passphrase_valid(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_03_a_ab_abc_abd_abf_abj() {
                let input = "a ab abc abd abf abj";
                let expected = true;
                let to_check = is_passphrase_valid(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_04_iiii_oiii_ooii_oooi_oooo() {
                let input = "iiii oiii ooii oooi oooo";
                let expected = true;
                let to_check = is_passphrase_valid(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_05_oiii_ioii_iioi_iiio() {
                let input = "oiii ioii iioi iiio";
                let expected = false;
                let to_check = is_passphrase_valid(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution() {
                const PUZZLE_INPUT: &'static str = include_str!("../input.txt");
                let expected = 265;
                let to_check = aoc_day04_part_2(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
