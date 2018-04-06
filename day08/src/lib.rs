#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;

#[derive(Debug, Default)]
struct Register {
    value: i32,
}

#[derive(Debug, Default)]
struct Registers<'a>(HashMap<&'a str, Register>);

#[derive(Debug, PartialEq, Clone)]
enum RegisterOperator {
    Add,
    Sub,
}

#[derive(Debug, PartialEq, Clone)]
enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(Debug)]
struct Operations<'a> {
    operations: std::iter::Peekable<std::str::Lines<'a>>,
    registers: Registers<'a>,
}

impl<'a> Operations<'a> {
    fn new(operations: &'a str) -> Operations {
        Operations {
            operations: operations.lines().peekable(),
            registers: Default::default(),
        }
    }

    fn parse_next_line(&mut self) -> bool {
        self.operations.next().map(|line| {
            let operation = Operation::from(line);

            let condition = {
                // Perform test on register value
                let cond_register = self.registers
                    .0
                    .entry(operation.condition_register)
                    .or_insert(Default::default());

                match operation.condition_operator {
                    ComparisonOperator::LessThan => cond_register.value < operation.condition_value,
                    ComparisonOperator::GreaterThan => {
                        cond_register.value > operation.condition_value
                    }
                    ComparisonOperator::Equal => cond_register.value == operation.condition_value,
                    ComparisonOperator::NotEqual => {
                        cond_register.value != operation.condition_value
                    }
                    ComparisonOperator::LessThanOrEqual => {
                        cond_register.value <= operation.condition_value
                    }
                    ComparisonOperator::GreaterThanOrEqual => {
                        cond_register.value >= operation.condition_value
                    }
                }
            };
            if condition {
                let target_register = self.registers
                    .0
                    .entry(operation.register_name)
                    .or_insert(Default::default());
                match operation.operation {
                    RegisterOperator::Add => target_register.value += operation.value_change,
                    RegisterOperator::Sub => target_register.value -= operation.value_change,
                }
            }
        });
        // Is there still some line(s) in `operations`?
        self.operations.peek().is_some()
    }

    fn largest_value(&self) -> i32 {
        self.registers
            .0
            .iter()
            .fold(0, |max_value, (_key, register)| {
                max_value.max(register.value)
            })
    }
}

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"(?x)      # b inc 5 if a > 1
(?P<register_name>\w+)          # Register name                 b
\s+                             # One or more spaces
(?P<operation>(inc|dec))        # Operation (inc/dec)           inc
\s+                             # One or more spaces
(?P<op_value>\-?\d+)            # Operation value               5
\s+                             # One or more spaces
if                              # Condition (begining)          if
\s+                             # One or more spaces
(?P<cond_reg_name>\w+)          # Condition (register name)     a
\s+                             # One or more spaces
(?P<cond_operator>[!<>=]{1,2})  # Condition (operator)          >
\s+                             # One or more spaces
(?P<cond_value>\-?\d+)          # Condition (value)             1
").unwrap();
}

#[derive(Debug)]
struct Operation<'a> {
    register_name: &'a str,
    operation: RegisterOperator,
    value_change: i32,
    condition_register: &'a str,
    condition_operator: ComparisonOperator,
    condition_value: i32,
}

impl<'a> From<&'a str> for Operation<'a> {
    fn from(line: &'a str) -> Self {
        let caps = RE.captures(line).unwrap();
        Operation {
            register_name: caps.name("register_name").unwrap().as_str(),
            operation: match &caps["operation"] {
                "inc" => RegisterOperator::Add,
                "dec" => RegisterOperator::Sub,
                _ => unreachable!(),
            },
            value_change: caps["op_value"].parse().unwrap(),
            condition_register: caps.name("cond_reg_name").unwrap().as_str(),
            condition_operator: match &caps["cond_operator"] {
                "<" => ComparisonOperator::LessThan,
                ">" => ComparisonOperator::GreaterThan,
                "==" => ComparisonOperator::Equal,
                "!=" => ComparisonOperator::NotEqual,
                "<=" => ComparisonOperator::LessThanOrEqual,
                ">=" => ComparisonOperator::GreaterThanOrEqual,
                _ => unreachable!(),
            },
            condition_value: caps["cond_value"].parse().unwrap(),
        }
    }
}

pub fn aoc_day08(instructions: &str) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day08 {
            use ::*;

            // const PUZZLE_INPUT: &'static str = include_str!("../input.txt");
            const EXAMPLE_01_INPUT: &str = "b inc 5 if a > 1
                                            a inc 1 if b < 5
                                            c dec -10 if a >= 1
                                            c inc -20 if c == 10";

            #[test]
            fn test_operation_from_regex_captures() {
                let operation = Operation::from("b inc 5 if a > 1");
                assert_eq!("b", operation.register_name);
                assert_eq!(RegisterOperator::Add, operation.operation);
                assert_eq!(5, operation.value_change);
                assert_eq!("a", operation.condition_register);
                assert_eq!(
                    ComparisonOperator::GreaterThan,
                    operation.condition_operator
                );
                assert_eq!(1, operation.condition_value);

                let operation = Operation::from("a inc 1 if b < 5");
                assert_eq!("a", operation.register_name);
                assert_eq!(RegisterOperator::Add, operation.operation);
                assert_eq!(1, operation.value_change);
                assert_eq!("b", operation.condition_register);
                assert_eq!(ComparisonOperator::LessThan, operation.condition_operator);
                assert_eq!(5, operation.condition_value);

                let operation = Operation::from("c dec -10 if a >= 1");
                assert_eq!("c", operation.register_name);
                assert_eq!(RegisterOperator::Sub, operation.operation);
                assert_eq!(-10, operation.value_change);
                assert_eq!("a", operation.condition_register);
                assert_eq!(
                    ComparisonOperator::GreaterThanOrEqual,
                    operation.condition_operator
                );
                assert_eq!(1, operation.condition_value);

                let operation = Operation::from("c inc -20 if c == 10");
                assert_eq!("c", operation.register_name);
                assert_eq!(RegisterOperator::Add, operation.operation);
                assert_eq!(-20, operation.value_change);
                assert_eq!("c", operation.condition_register);
                assert_eq!(ComparisonOperator::Equal, operation.condition_operator);
                assert_eq!(10, operation.condition_value);
            }

            #[test]
            fn test_operations() {
                let mut operations = Operations::new(EXAMPLE_01_INPUT);

                let is_not_done = operations.parse_next_line();
                assert!(is_not_done);
                assert_eq!(0, operations.registers.0.get("a").unwrap().value);
                assert!(operations.registers.0.get("b").is_none());
                assert_eq!(0, operations.largest_value());

                let is_not_done = operations.parse_next_line();
                assert!(is_not_done);
                assert_eq!(1, operations.registers.0.get("a").unwrap().value);
                assert_eq!(0, operations.registers.0.get("b").unwrap().value);
                assert_eq!(1, operations.largest_value());

                let is_not_done = operations.parse_next_line();
                assert!(is_not_done);
                assert_eq!(1, operations.registers.0.get("a").unwrap().value);
                assert_eq!(0, operations.registers.0.get("b").unwrap().value);
                assert_eq!(10, operations.registers.0.get("c").unwrap().value);
                assert_eq!(10, operations.largest_value());

                let is_not_done = operations.parse_next_line();
                assert!(!is_not_done);
                assert_eq!(1, operations.registers.0.get("a").unwrap().value);
                assert_eq!(0, operations.registers.0.get("b").unwrap().value);
                assert_eq!(-10, operations.registers.0.get("c").unwrap().value);
                assert_eq!(1, operations.largest_value());
            }

            // #[test]
            // fn parse() {
            //     let mut operations = Operations::new(EXAMPLE_01_INPUT);
            //     operations.parse_next_line();
            //     // let reg: Register = Default::default();
            //     // println!("reg: {:#?}", reg);
            //     // parse_input(EXAMPLE_01_INPUT);
            // }

            #[test]
            fn example_01_steps() {}

            #[test]
            fn example_01() {}

            #[test]
            fn solution() {
                // let expected = ???;
                // let input = ???;
                // let to_check = aoc_day08(PUZZLE_INPUT);

                // assert_eq!(expected, to_check);
            }
        }
    }
}
