use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Program<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<&'a str>,
}

#[derive(Debug)]
struct Node<'a> {
    program: &'a Program<'a>,
    children: Vec<Node<'a>>,
}

fn parse_input<'a>(input: &'a str) -> Vec<Program<'a>> {
    input
        .lines()
        .map(|line| {
            let mut word_iter = line.split_whitespace();
            let program_name = word_iter.next().unwrap();
            let weight = word_iter
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
                weight: weight,
                children: children,
            }
        })
        .collect()
}

fn find_bottom_program<'a>(programs: &'a [Program<'a>]) -> Node<'a> {
    // First, create nodes for all programs that don't have
    // any children. Those are the leaf nodes.
    // let mut trees: Vec<Node> = programs
    let mut trees: HashMap<&'a str, Node> = programs
        .iter()
        .map(|program| {
            (
                program.name,
                Node {
                    program: &program,
                    children: Vec::new(),
                },
            )
        })
        .collect();

    // The `trees` now contains all programs but flat. Go over every one of them
    // and remove their children from `trees`.
    programs
        .iter()
        // Don't loop over programs without children
        .filter(|parent_program| !parent_program.children.is_empty())
        .for_each(|parent_program| {
            // Find the children nodes and remove them from the `trees`.
            parent_program
                .children
                .iter()
                // Remove the child node from `trees`
                .for_each(|child_name| { trees.remove(child_name).unwrap(); });
        });

    let (_root_name, root_node) = trees.into_iter().nth(0).unwrap();

    root_node
}

pub fn aoc_day07<'a>(input: &'a str) -> &'a str {
    let programs = parse_input(input);

    let root_node = find_bottom_program(&programs);
    // A member variable of `root_node` cannot be returned since it does
    // not live long enough; `root_node` (and thus its member variables too)
    // are part of the current stack frame and will disappear when the
    // function returns.
    // Instead, return the original string reference as found in `programs`.
    // This saves an allocation, at the cost of having to find the proper name.
    programs
        .iter()
        .find(|program| program.name == root_node.program.name)
        .unwrap()
        .name
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
                        weight: 66,
                        children: vec![],
                    },
                    Program {
                        name: "xhth",
                        weight: 57,
                        children: vec![],
                    },
                    Program {
                        name: "ebii",
                        weight: 61,
                        children: vec![],
                    },
                    Program {
                        name: "havc",
                        weight: 66,
                        children: vec![],
                    },
                    Program {
                        name: "ktlj",
                        weight: 57,
                        children: vec![],
                    },
                    Program {
                        name: "fwft",
                        weight: 72,
                        children: vec!["ktlj", "cntj", "xhth"],
                    },
                    Program {
                        name: "qoyq",
                        weight: 66,
                        children: vec![],
                    },
                    Program {
                        name: "padx",
                        weight: 45,
                        children: vec!["pbga", "havc", "qoyq"],
                    },
                    Program {
                        name: "tknk",
                        weight: 41,
                        children: vec!["ugml", "padx", "fwft"],
                    },
                    Program {
                        name: "jptl",
                        weight: 61,
                        children: vec![],
                    },
                    Program {
                        name: "ugml",
                        weight: 68,
                        children: vec!["gyxo", "ebii", "jptl"],
                    },
                    Program {
                        name: "gyxo",
                        weight: 61,
                        children: vec![],
                    },
                    Program {
                        name: "cntj",
                        weight: 57,
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
                const PUZZLE_INPUT: &'static str = include_str!("../day07_input.txt");
                let expected = "wiapj";
                let to_check = aoc_day07(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
