#[derive(Debug, PartialEq)]
struct Program<'a> {
    name: &'a str,
    nb_disks: u32,
    children: Vec<&'a str>,
}

fn parse_input<'a>(input: &'a str) -> Vec<Program<'a>> {
    input
        .lines()
        .map(|line| {
            let mut word_iter = line.split_whitespace();
            let program_name = word_iter.next().unwrap();
            let nb_disks = word_iter
                .next()
                .unwrap()
                .trim_matches('(')
                .trim_matches(')')
                .parse()
                .unwrap();
            let children = if let Some(_arrow) = word_iter.next() {
                word_iter.map(|word| word.trim_matches(',')).collect()
            } else {
                Vec::new()
            };
            Program {
                name: program_name,
                nb_disks: nb_disks,
                children: children,
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
            fn example1_program_build_vec<'a>() -> Vec<Program<'a>> {
                vec![
                    Program {
                        name: "pbga",
                        nb_disks: 66,
                        children: vec![],
                    },
                    Program {
                        name: "xhth",
                        nb_disks: 57,
                        children: vec![],
                    },
                    Program {
                        name: "ebii",
                        nb_disks: 61,
                        children: vec![],
                    },
                    Program {
                        name: "havc",
                        nb_disks: 66,
                        children: vec![],
                    },
                    Program {
                        name: "ktlj",
                        nb_disks: 57,
                        children: vec![],
                    },
                    Program {
                        name: "fwft",
                        nb_disks: 72,
                        children: vec!["ktlj", "cntj", "xhth"],
                    },
                    Program {
                        name: "qoyq",
                        nb_disks: 66,
                        children: vec![],
                    },
                    Program {
                        name: "padx",
                        nb_disks: 45,
                        children: vec!["pbga", "havc", "qoyq"],
                    },
                    Program {
                        name: "tknk",
                        nb_disks: 41,
                        children: vec!["ugml", "padx", "fwft"],
                    },
                    Program {
                        name: "jptl",
                        nb_disks: 61,
                        children: vec![],
                    },
                    Program {
                        name: "ugml",
                        nb_disks: 68,
                        children: vec!["gyxo", "ebii", "jptl"],
                    },
                    Program {
                        name: "gyxo",
                        nb_disks: 61,
                        children: vec![],
                    },
                    Program {
                        name: "cntj",
                        nb_disks: 57,
                        children: vec![],
                    },
                ]
            }

            #[test]
            fn example_01_parse_input() {
                let to_check = parse_input(EXAMPLE1);
                let expected = example1_program_build_vec();
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
