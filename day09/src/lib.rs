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
