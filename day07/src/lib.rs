//! # Day 7: Recursive Circus
//!
//! ## Part One
//!
//! Wandering further through the circuits of the computer, you come upon a tower of programs that have gotten themselves into a bit of trouble. A recursive algorithm has gotten out of hand, and now they're balanced precariously in a large tower.
//!
//! One program at the bottom supports the entire tower. It's holding a large disc, and on the disc are balanced several more sub-towers. At the bottom of these sub-towers, standing on the bottom disc, are other programs, each holding _their_ own disc, and so on. At the very tops of these sub-sub-sub-...-towers, many programs stand simply keeping the disc below them balanced but with no disc of their own.
//!
//! You offer to help, but first you need to understand the structure of these towers. You ask each program to yell out their _name_, their _weight_, and (if they're holding a disc) the _names of the programs immediately above them_ balancing on that disc. You write this information down (your puzzle input). Unfortunately, in their panic, they don't do this in an orderly fashion; by the time you're done, you're not sure which program gave which information.
//!
//! For example, if your list is the following:
//!
//!```text
//!     pbga (66)
//!     xhth (57)
//!     ebii (61)
//!     havc (66)
//!     ktlj (57)
//!     fwft (72) -> ktlj, cntj, xhth
//!     qoyq (66)
//!     padx (45) -> pbga, havc, qoyq
//!     tknk (41) -> ugml, padx, fwft
//!     jptl (61)
//!     ugml (68) -> gyxo, ebii, jptl
//!     gyxo (61)
//!     cntj (57)
//!```
//!
//! ...then you would be able to recreate the structure of the towers that looks like this:
//!
//!```text
//!                     gyxo
//!                   /
//!              ugml - ebii
//!            /      \
//!           |         jptl
//!           |
//!           |         pbga
//!          /        /
//!     tknk --- padx - havc
//!          \        \
//!           |         qoyq
//!           |
//!           |         ktlj
//!            \      /
//!              fwft - cntj
//!                   \
//!                     xhth
//!```
//!
//! In this example, `tknk` is at the bottom of the tower (the _bottom program_), and is holding up `ugml`, `padx`, and `fwft`. Those programs are, in turn, holding up other programs; in this example, none of those programs are holding up any other programs, and are all the tops of their own towers. (The actual tower balancing in front of you is much larger.)
//!
//! Before you're ready to help them, you need to make sure your information is correct. _What is the name of the bottom program?_
//!
//! ## Part Two
//!
//! The programs explain the situation: they can't get down. Rather, they _could_ get down, if they weren't expending all of their energy trying to keep the tower balanced. Apparently, one program has the _wrong weight_, and until it's fixed, they're stuck here.
//!
//! For any program holding a disc, each program standing on that disc forms a sub-tower. Each of those sub-towers are supposed to be the same weight, or the disc itself isn't balanced. The weight of a tower is the sum of the weights of the programs in that tower.
//!
//! In the example above, this means that for `ugml`'s disc to be balanced, `gyxo`, `ebii`, and `jptl` must all have the same weight, and they do: `61`.
//!
//! However, for `tknk` to be balanced, each of the programs standing on its disc _and all programs above it_ must each match. This means that the following sums must all be the same:
//!
//! *   `ugml` \+ (`gyxo` \+ `ebii` \+ `jptl`) = 68 + (61 + 61 + 61) = 251
//! *   `padx` \+ (`pbga` \+ `havc` \+ `qoyq`) = 45 + (66 + 66 + 66) = 243
//! *   `fwft` \+ (`ktlj` \+ `cntj` \+ `xhth`) = 72 + (57 + 57 + 57) = 243
//!
//! As you can see, `tknk`'s disc is unbalanced: `ugml`'s stack is heavier than the other two. Even though the nodes above `ugml` are balanced, `ugml` itself is too heavy: it needs to be `8` units lighter for its stack to weigh `243` and keep the towers balanced. If this change were made, its weight would be `60`.
//!
//! Given that exactly one program is the wrong weight, _what would its weight need to be_ to balance the entire tower?

extern crate indexmap;

use std::collections::HashMap;

use indexmap::IndexMap;

#[derive(Clone, Debug)]
struct TempNode<'a> {
    weight: u32,
    children: Vec<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct Node<'a> {
    name: &'a str,
    weight: u32,

    children: Vec<Node<'a>>,
    total_weight: u32,
}

impl<'a> Node<'a> {
    fn propagate_weights(&mut self) {
        let children_weight: u32 = self.children
            .iter_mut()
            .map(|child| {
                child.propagate_weights();
                child.total_weight
            })
            .sum();
        self.total_weight = self.weight + children_weight;

        // Sort children. The unbalanced child, if any, will be at either end.
        self.children
            .sort_unstable_by(|a, b| a.total_weight.cmp(&b.total_weight));
    }

    #[allow(dead_code)]
    fn find_node(&self, node_name: &str) -> Option<(Option<&Node<'a>>, &Node<'a>)> {
        if self.name == node_name {
            Some((None, &self))
        } else {
            for child in &self.children {
                if let Some(mut found) = child.find_node(node_name) {
                    if found.0.is_none() {
                        found.0 = Some(&self);
                    }
                    return Some(found);
                }
            }
            None
        }
    }
}

pub fn build_tree<'a>(input: &'a str) -> Node<'a> {
    // Create a hash table of temporary tree nodes (`TempNode`) by parsing the input string.
    // Use an `indexmap::IndexMap` instead of a `std::collections::HashMap` to bypass
    // a borrowck issue: This hash table will be drained by iterating over it _and_
    // when a node's child needs to be created.
    let mut unparsed_nodes: IndexMap<&'a str, TempNode<'a>> = input
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
            (
                program_name,
                TempNode {
                    weight: weight,
                    children: children,
                },
            )
        })
        .collect();

    // Orphan nodes are `Node`s without parents. The parent of a node might appear
    // after its child in the input string.
    let mut orphan_nodes: HashMap<&'a str, Node<'a>> = HashMap::new();

    while let Some((unparsed_node_name, unparsed_node)) = unparsed_nodes.pop() {
        // Try to find an orphan node
        let node_exists = orphan_nodes.contains_key(unparsed_node_name);
        if !node_exists {
            // Create a `Node` and insert it as orphan since its parent is unknown (yet)
            let new_node = build_real_node(
                unparsed_node_name,
                unparsed_node,
                &mut unparsed_nodes,
                &mut orphan_nodes,
            );

            orphan_nodes.insert(unparsed_node_name, new_node);
        }
    }

    // Only one node should be orphan: the tree's root node
    assert_eq!(1, orphan_nodes.len());
    assert!(unparsed_nodes.is_empty());

    let (_root_name, mut root_node) = orphan_nodes.into_iter().nth(0).unwrap();

    root_node.propagate_weights();

    root_node
}

fn build_real_node<'a>(
    name: &'a str,
    node: TempNode<'a>,
    unparsed_nodes: &mut IndexMap<&'a str, TempNode<'a>>,
    orphan_nodes: &mut HashMap<&'a str, Node<'a>>,
) -> Node<'a> {
    let child_nodes = node.children
        .iter()
        .map(|child_name| {
            // Remove the child `TempNode`s from the hashmap
            // and build a new `Node` recursively.
            // If `child_name` is not present in `unparsed_nodes` check `orphan_nodes` instead
            if let Some(child_node) = unparsed_nodes.remove(child_name) {
                build_real_node(child_name, child_node, unparsed_nodes, orphan_nodes)
            } else {
                orphan_nodes.remove(child_name).unwrap()
            }
        })
        .collect();

    Node {
        name: name,
        weight: node.weight,
        children: child_nodes,
        total_weight: 0,
    }
}

fn find_first_unbalenced<'a>(parent_node: &'a Node) -> &'a Node<'a> {
    assert!(parent_node.children.len() > 2);

    let child_iter = parent_node.children.iter();

    let first_child = child_iter.clone().nth(0).unwrap();
    let last_child = child_iter.clone().last().unwrap();

    // Children were sorted by weight when the tree was built
    assert!(first_child.total_weight <= last_child.total_weight);

    if first_child.total_weight == last_child.total_weight {
        // End of recursion: Children all have the same total weight.
        // The unbalanced node is thus the current one.
        parent_node
    } else {
        // Check which child (first or last) is unbalanced and recurse
        let second_child = child_iter.clone().nth(1).unwrap();

        if first_child.total_weight != second_child.total_weight {
            find_first_unbalenced(first_child)
        } else {
            find_first_unbalenced(last_child)
        }
    }
}

fn find_unbalenced_child<'a>(parent_node: &'a Node) -> (Option<&'a Node<'a>>, &'a Node<'a>) {
    let unbalenced_child = find_first_unbalenced(parent_node);
    // Since we don't store pointers to parent node, lets simply
    // perform a lookup for the found node, which will return the parent node too.
    let (unbalanced_parent, _unbalenced_child) =
        parent_node.find_node(unbalenced_child.name).unwrap();
    (unbalanced_parent, unbalenced_child)
}

pub fn aoc_day07_part_1<'a>(input: &'a str) -> &'a str {
    let root_node = build_tree(input);

    root_node.name
}

pub fn aoc_day07_part_2(input: &str) -> i32 {
    let root_node = build_tree(input);

    // Find the unbalanced node
    // Find the deepest node for which children don't share the same weight
    let (unbalanced_parent, unbalenced_child) = find_unbalenced_child(&root_node);
    let unbalanced_parent = unbalanced_parent.unwrap();

    // Now find by how much the unbalanced child is
    let diff = unbalanced_parent
        .children
        .iter()
        .filter(|child| child.name != unbalenced_child.name)
        .nth(0)
        .unwrap()
        .total_weight as i32 - unbalenced_child.total_weight as i32;

    unbalenced_child.weight as i32 + diff
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day07 {
            use ::*;

            const PUZZLE_INPUT: &'static str = include_str!("../input.txt");

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

            #[test]
            fn part_1_example_01_build_tree() {
                let expected = "tknk";
                let root_node = build_tree(EXAMPLE1);
                let to_check = root_node.name;

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_1_example_01() {
                let expected = "tknk";
                let to_check = aoc_day07_part_1(EXAMPLE1);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_example_01_propagate_weights() {
                let root_node = build_tree(EXAMPLE1);

                let expected_weight: HashMap<&str, u32> =
                    [("ugml", 251), ("padx", 243), ("fwft", 243)]
                        .iter()
                        .cloned()
                        .collect();

                for child in root_node.children.iter() {
                    let expected = *expected_weight.get(child.name).unwrap();
                    let to_check = child.total_weight;
                    assert_eq!(expected, to_check);
                }
            }

            #[test]
            fn part_2_example_01_find_unbalenced_child() {
                let root_node = build_tree(EXAMPLE1);
                let (unbalanced_parent, unbalenced_child) = find_unbalenced_child(&root_node);

                let to_check = unbalanced_parent.unwrap().name;
                let expected = "tknk";
                assert_eq!(expected, to_check);

                let to_check = unbalenced_child.name;
                let expected = "ugml";
                assert_eq!(expected, to_check);

                let to_check = unbalenced_child.total_weight;
                let expected = 251;
                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_example_01_find_node() {
                let root_node = build_tree(EXAMPLE1);

                let found = root_node.find_node("asjdaksdhakd");
                assert!(found.is_none());

                let (found_parent, tknk) = root_node.find_node("tknk").unwrap();
                assert!(found_parent.is_none());
                assert_eq!(tknk.name, "tknk");
                assert_eq!(tknk.weight, 41);
                assert_eq!(tknk.total_weight, 778);

                let (found_parent, ugml) = root_node.find_node("ugml").unwrap();
                assert_eq!(*found_parent.unwrap(), *tknk);
                assert_eq!(ugml.name, "ugml");
                assert_eq!(ugml.weight, 68);
                assert_eq!(ugml.total_weight, 251);

                let (found_parent, padx) = root_node.find_node("padx").unwrap();
                assert_eq!(*found_parent.unwrap(), *tknk);
                assert_eq!(padx.name, "padx");
                assert_eq!(padx.weight, 45);
                assert_eq!(padx.total_weight, 243);

                let (found_parent, fwft) = root_node.find_node("fwft").unwrap();
                assert_eq!(*found_parent.unwrap(), *tknk);
                assert_eq!(fwft.name, "fwft");
                assert_eq!(fwft.weight, 72);
                assert_eq!(fwft.total_weight, 243);

                for node_name in &["gyxo", "ebii", "jptl"] {
                    let (found_parent, found_node) = root_node.find_node(node_name).unwrap();
                    assert_eq!(*found_parent.unwrap(), *ugml);
                    assert_eq!(found_node.name, *node_name);
                    assert_eq!(found_node.weight, 61);
                    assert_eq!(found_node.total_weight, 61);
                }
                for node_name in &["pbga", "havc", "qoyq"] {
                    let (found_parent, found_node) = root_node.find_node(node_name).unwrap();
                    assert_eq!(*found_parent.unwrap(), *padx);
                    assert_eq!(found_node.name, *node_name);
                    assert_eq!(found_node.weight, 66);
                    assert_eq!(found_node.total_weight, 66);
                }
                for node_name in &["ktlj", "cntj", "xhth"] {
                    let (found_parent, found_node) = root_node.find_node(node_name).unwrap();
                    assert_eq!(*found_parent.unwrap(), *fwft);
                    assert_eq!(found_node.name, *node_name);
                    assert_eq!(found_node.weight, 57);
                    assert_eq!(found_node.total_weight, 57);
                }
            }

            #[test]
            fn part_2_example_01() {
                let to_check = aoc_day07_part_2(EXAMPLE1);
                let expected = 60;
                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_1_solution() {
                let expected = "wiapj";
                let to_check = aoc_day07_part_1(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn part_2_solution() {
                let expected = 1072;
                let to_check = aoc_day07_part_2(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
                assert!(to_check != 87851);
            }
        }
    }
}
