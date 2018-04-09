//! # Day 9: Stream Processing
//!
//! ## Part One
//!
//! A large stream blocks your path. According to the locals, it's not safe to cross the stream at the moment because it's full of _garbage_. You look down at the stream; rather than water, you discover that it's a _stream of characters_.
//!
//! You sit for a while and record part of the stream (your puzzle input). The characters represent _groups_ \- sequences that begin with `{` and end with `}`. Within a group, there are zero or more other things, separated by commas: either another _group_ or _garbage_. Since groups can contain other groups, a `}` only closes the _most-recently-opened unclosed group_ \- that is, they are nestable. Your puzzle input represents a single, large group which itself contains many smaller ones.
//!
//! Sometimes, instead of a group, you will find _garbage_. Garbage begins with `<` and ends with `>`. Between those angle brackets, almost any character can appear, including `{` and `}`. _Within_ garbage, `<` has no special meaning.
//!
//! In a futile attempt to clean up the garbage, some program has _canceled_ some of the characters within it using `!`: inside garbage, _any_ character that comes after `!` should be _ignored_, including `<`, `>`, and even another `!`.
//!
//! You don't see any characters that deviate from these rules. Outside garbage, you only find well-formed groups, and garbage always terminates according to the rules above.
//!
//! Here are some self-contained pieces of garbage:
//!
//! *   `<>`, empty garbage.
//! *   `<random characters>`, garbage containing random characters.
//! *   `<<<<>`, because the extra `<` are ignored.
//! *   `<{!>}>`, because the first `>` is canceled.
//! *   `<!!>`, because the second `!` is canceled, allowing the `>` to terminate the garbage.
//! *   `<!!!>>`, because the second `!` and the first `>` are canceled.
//! *   `<{o"i!a,<{i<a>`, which ends at the first `>`.
//!
//! Here are some examples of whole streams and the number of groups they contain:
//!
//! *   `{}`, `1` group.
//! *   `{{{}}}`, `3` groups.
//! *   `{{},{}}`, also `3` groups.
//! *   `{{{},{},{{}}}}`, `6` groups.
//! *   `{<{},{},{{}}>}`, `1` group (which itself contains garbage).
//! *   `{<a>,<a>,<a>,<a>}`, `1` group.
//! *   `{{<a>},{<a>},{<a>},{<a>}}`, `5` groups.
//! *   `{{<!>},{<!>},{<!>},{<a>}}`, `2` groups (since all but the last `>` are canceled).
//!
//! Your goal is to find the total score for all groups in your input. Each group is assigned a _score_ which is one more than the score of the group that immediately contains it. (The outermost group gets a score of `1`.)
//!
//! *   `{}`, score of `1`.
//! *   `{{{}}}`, score of `1 + 2 + 3 = 6`.
//! *   `{{},{}}`, score of `1 + 2 + 2 = 5`.
//! *   `{{{},{},{{}}}}`, score of `1 + 2 + 3 + 3 + 3 + 4 = 16`.
//! *   `{<a>,<a>,<a>,<a>}`, score of `1`.
//! *   `{{<ab>},{<ab>},{<ab>},{<ab>}}`, score of `1 + 2 + 2 + 2 + 2 = 9`.
//! *   `{{<!!>},{<!!>},{<!!>},{<!!>}}`, score of `1 + 2 + 2 + 2 + 2 = 9`.
//! *   `{{<a!>},{<a!>},{<a!>},{<ab>}}`, score of `1 + 2 = 3`.
//!
//! _What is the total score_ for all groups in your input?
//!
//! ## Part Two
//!
//! Now, you're ready to remove the garbage.
//!
//! To prove you've removed it, you need to count all of the characters within the garbage. The leading and trailing `<` and `>` don't count, nor do any canceled characters or the `!` doing the canceling.
//!
//! *   `<>`, `0` characters.
//! *   `<random characters>`, `17` characters.
//! *   `<<<<>`, `3` characters.
//! *   `<{!>}>`, `2` characters.
//! *   `<!!>`, `0` characters.
//! *   `<!!!>>`, `0` characters.
//! *   `<{o"i!a,<{i<a>`, `10` characters.
//!
//! _How many non-canceled characters are within the garbage_ in your puzzle input?

pub fn aoc_day09(input: &str) -> (Vec<(usize, usize)>, usize, usize) {
    let mut stack_start_group = Vec::<usize>::new();
    let mut spans_group = Vec::<(usize, usize)>::new();
    let mut total_score = 0;
    let mut nested_level = 0;

    let mut nb_garbage_chars = 0;
    let mut is_garbage = false;
    let mut next_is_ignored = false;
    let mut it = input.chars().enumerate();
    while let Some((i, c)) = it.next() {
        if next_is_ignored {
            next_is_ignored = false;
        } else {
            match c {
                '!' => next_is_ignored = true,
                _ => {
                    if is_garbage {
                        nb_garbage_chars += 1;
                    }
                    match c {
                        '<' => {
                            is_garbage = true;
                        }
                        '>' => {
                            is_garbage = false;
                            nb_garbage_chars -= 1;
                        }
                        '{' => {
                            if !is_garbage {
                                stack_start_group.push(i);
                                nested_level += 1;
                            }
                        }
                        '}' => {
                            if !is_garbage {
                                spans_group.push((stack_start_group.pop().unwrap(), i));
                                total_score += nested_level;
                                nested_level -= 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    (spans_group, total_score, nb_garbage_chars)
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day09 {
            use ::*;

            //                      (input, group_count)
            const EXAMPLE_O1_COUNT: [(&str, usize); 8] = [
                ("{}", 1),
                ("{{{}}}", 3),
                ("{{},{}}", 3),
                ("{{{},{},{{}}}}", 6),
                ("{<{},{},{{}}>}", 1),
                ("{<a>,<a>,<a>,<a>}", 1),
                ("{{<a>},{<a>},{<a>},{<a>}}", 5),
                ("{{<!>},{<!>},{<!>},{<a>}}", 2),
            ];
            const EXAMPLE_EXTRAS_COUNT: [(&str, usize); 4] = [
                ("{<!!>}", 1),
                ("{{<!!>}}", 2),
                ("{{<!!>},{<!!>}}", 3),
                ("{{<!!>},{{<!!>}}}", 4),
            ];

            //                      (input, score)
            const EXAMPLE_O1_SCORE: [(&str, usize); 8] = [
                ("{}", 1),
                ("{{{}}}", 6),
                ("{{},{}}", 5),
                ("{{{},{},{{}}}}", 16),
                ("{<a>,<a>,<a>,<a>}", 1),
                ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
                ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
                ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
            ];

            const EXAMPLE_O4_GARBAGE: [(&str, usize); 7] = [
                ("<>", 0),
                ("<random characters>", 17),
                ("<<<<>", 3),
                ("<{!>}>", 2),
                ("<!!>", 0),
                ("<!!!>>", 0),
                (r#"<{o"i!a,<{i<a>"#, 10),
            ];

            const PUZZLE_INPUT: &'static str = include_str!("../input");

            #[test]
            fn example_01_count() {
                for &(input, expected_count) in EXAMPLE_O1_COUNT.iter() {
                    let (spans_group, _, _) = aoc_day09(input);
                    let to_check_count = spans_group.len();
                    assert_eq!(expected_count, to_check_count);
                }
            }

            #[test]
            fn example_extras_count() {
                for &(input, expected_count) in EXAMPLE_EXTRAS_COUNT.iter() {
                    let (spans_group, _, _) = aoc_day09(input);
                    let to_check_count = spans_group.len();
                    assert_eq!(expected_count, to_check_count);
                }
            }

            #[test]
            fn example_01_score() {
                for &(input, expected_score) in EXAMPLE_O1_SCORE.iter() {
                    let (_, to_check_score, _) = aoc_day09(input);
                    assert_eq!(expected_score, to_check_score);
                }
            }

            #[test]
            fn example_04_garbage_count() {
                for &(input, expected) in EXAMPLE_O4_GARBAGE.iter() {
                    let (_, _, to_check) = aoc_day09(input);
                    assert_eq!(expected, to_check);
                }
            }

            #[test]
            fn solution_part_1() {
                let expected = 12897;
                let (_, to_check, _) = aoc_day09(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution_part_2() {
                let expected = 7031;
                let (_, _, to_check) = aoc_day09(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
