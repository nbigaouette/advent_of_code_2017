#[derive(Debug, PartialEq)]
struct Program<'a> {
    name: &'a str,
    nb_disks: u32,
}

fn parse_input<'a>(input: &'a str) -> Vec<Program<'a>> {
    input
        .lines()
        .map(|line| {
            let mut word_iter = line.split_whitespace();
            let program_name = word_iter.next().unwrap();
            let nb_disks: u32 = word_iter
                .next()
                .unwrap()
                .trim_matches('(')
                .trim_matches(')')
                .parse()
                .unwrap();
            Program {
                name: program_name,
                nb_disks: nb_disks,
            }
        })
        .collect()
}

pub fn aoc_day07(input: &str) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day07 {
            use ::*;

            const EXAMPLE1: &str = "pbga (66)
                                    xhth (57)
                                    ebii (61)
                                    havc (66)
                                    ktlj (57)
                                    fwft (72) -> ktlj, cntj, xhth
                                    qoyq (66)
                                    padx (45) -> pbga, havc, qoyq
                                    tknk (41) -> ugml, padx, fwft
                                    jptl (61)
                                    ugml (68) -> gyxo, ebii, jptl
                                    gyxo (61)
                                    cntj (57)";
            /*
                          gyxo
                        /
                    ugml - ebii
                /      \
                |         jptl
                |
                |         pbga
                /        /
            tknk --- padx - havc
                \        \
                |         qoyq
                |
                |         ktlj
                \      /
                    fwft - cntj
                        \
                            xhth
            */

            #[test]
            fn example_01_parse_input() {
                let to_check = parse_input(EXAMPLE1);
                let expected = vec![
                    Program {
                        name: "pbga",
                        nb_disks: 66,
                    },
                    Program {
                        name: "xhth",
                        nb_disks: 57,
                    },
                    Program {
                        name: "ebii",
                        nb_disks: 61,
                    },
                    Program {
                        name: "havc",
                        nb_disks: 66,
                    },
                    Program {
                        name: "ktlj",
                        nb_disks: 57,
                    },
                    Program {
                        name: "fwft",
                        nb_disks: 72,
                    },
                    Program {
                        name: "qoyq",
                        nb_disks: 66,
                    },
                    Program {
                        name: "padx",
                        nb_disks: 45,
                    },
                    Program {
                        name: "tknk",
                        nb_disks: 41,
                    },
                    Program {
                        name: "jptl",
                        nb_disks: 61,
                    },
                    Program {
                        name: "ugml",
                        nb_disks: 68,
                    },
                    Program {
                        name: "gyxo",
                        nb_disks: 61,
                    },
                    Program {
                        name: "cntj",
                        nb_disks: 57,
                    },
                ];
                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_01() {
                let expected = "tknk";
                let to_check = aoc_day07(EXAMPLE1);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution() {
                // const PUZZLE_INPUT: &'static str = include_str!("../day07_input.txt");
                // let expected = ???;
                // let to_check = aoc_day07(PUZZLE_INPUT);

                // assert_eq!(expected, to_check);
            }
        }
    }
}
