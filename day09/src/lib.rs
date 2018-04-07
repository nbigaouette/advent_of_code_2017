pub fn aoc_day09(input: &str) -> (Vec<(usize, usize)>, usize) {
    let mut bracket_stack_pos = Vec::<usize>::new();
    let mut group_spans = Vec::<(usize, usize)>::new();
    let mut total_score = 0;
    let mut nested_level = 0;

    let mut is_garbage = false;
    let mut next_is_ignored = false;
    let mut it = input.chars().enumerate();
    while let Some((i, c)) = it.next() {
        if next_is_ignored {
            next_is_ignored = false;
        } else {
            match c {
                '!' => next_is_ignored = true,
                _ => match c {
                    '<' => is_garbage = true,
                    '>' => is_garbage = false,
                    '{' => {
                        if !is_garbage {
                            bracket_stack_pos.push(i);
                            nested_level += 1;
                        }
                    }
                    '}' => {
                        if !is_garbage {
                            group_spans.push((bracket_stack_pos.pop().unwrap(), i));
                            total_score += nested_level;
                            nested_level -= 1;
                        }
                    }
                    _ => {}
                },
            }
        }
    }

    (group_spans, total_score)
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

            const PUZZLE_INPUT: &'static str = include_str!("../input");

            #[test]
            fn example_01_count() {
                for &(input, expected_count) in EXAMPLE_O1_COUNT.iter() {
                    let (group_spans, _) = aoc_day09(input);
                    let to_check_count = group_spans.len();
                    assert_eq!(expected_count, to_check_count);
                }
            }

            #[test]
            fn example_extras_count() {
                for &(input, expected_count) in EXAMPLE_EXTRAS_COUNT.iter() {
                    let (group_spans, _) = aoc_day09(input);
                    let to_check_count = group_spans.len();
                    assert_eq!(expected_count, to_check_count);
                }
            }

            #[test]
            fn example_01_score() {
                for &(input, expected_score) in EXAMPLE_O1_SCORE.iter() {
                    let (_, to_check_score) = aoc_day09(input);
                    assert_eq!(expected_score, to_check_score);
                }
            }

            #[test]
            fn solution_part_1() {
                let expected = 12897;
                let (_, to_check) = aoc_day09(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
