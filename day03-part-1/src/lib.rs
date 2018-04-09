//! # Day 3: Spiral Memory
//!
//! ## Part One
//!
//! You come across an experimental new kind of memory stored on an infinite two-dimensional grid.
//!
//! Each square on the grid is allocated in a spiral pattern starting at a location marked `1` and then counting up while spiraling outward. For example, the first few squares are allocated like this:
//!
//!```text
//!     17  16  15  14  13
//!     18   5   4   3  12
//!     19   6   1   2  11
//!     20   7   8   9  10
//!     21  22  23---> ...
//!```
//!
//! While this is very space-efficient (no squares are skipped), requested data must be carried back to square `1` (the location of the only access port for this memory system) by programs that can only move up, down, left, or right. They always take the shortest path: the [Manhattan Distance](https://en.wikipedia.org/wiki/Taxicab_geometry) between the location of the data and square `1`.
//!
//! For example:
//!
//! * Data from square `1` is carried `0` steps, since it's at the access port.
//! * Data from square `12` is carried `3` steps, such as: down, left, left.
//! * Data from square `23` is carried only `2` steps: up twice.
//! * Data from square `1024` must be carried `31` steps.
//!
//! _How many steps_ are required to carry the data from the square identified in your puzzle input all the way to the access port?
//!
//! ## Part Two
//!
//! As a stress test on the system, the programs here clear the grid and then store the value `1` in square `1`. Then, in the same allocation order as shown above, they store the sum of the values in all adjacent squares, including diagonals.
//!
//! So, the first few squares' values are chosen as follows:
//!
//! * Square `1` starts with the value `1`.
//! * Square `2` has only one adjacent filled square (with value `1`), so it also stores `1`.
//! * Square `3` has both of the above squares as neighbors and stores the sum of their values, `2`.
//! * Square `4` has all three of the aforementioned squares as neighbors and stores the sum of their values, `4`.
//! * Square `5` only has the first and fourth squares as neighbors, so it gets the value `5`.
//!
//! Once a square is written, its value does not change. Therefore, the first few squares would receive the following values:
//!
//!```text
//!     147  142  133  122   59
//!     304    5    4    2   57
//!     330   10    1    1   54
//!     351   11   23   25   26
//!     362  747  806--->   ...
//!```
//!
//! What is the _first value written_ that is _larger_ than your puzzle input?

fn spiral_id(n: i32) -> i32 {
    (((n as f32).sqrt() + 1.0) / 2.0).ceil() as i32
}

fn spiral_width(id: i32) -> i32 {
    2 * (id - 1) + 1
}

fn spiral_first_val(id: i32) -> i32 {
    ((id - 1) * 2 - 1).max(0).pow(2) + 1
}

fn spiral_side_id(n: i32, first_val: i32, width: i32) -> i32 {
    (n - first_val) / (width - 1)
}

pub fn aoc_day03_part_1(n: u32) -> u32 {
    if n == 1 {
        0
    } else {
        let n = n as i32;

        // Spiral's id
        let a = spiral_id(n);
        // Square's width
        let w = spiral_width(a);
        // First number of the spiral
        let i = spiral_first_val(a);
        // Last number of the spiral
        // let j = (a * 2 - 1).pow(2);
        // Spiral's / square's side id
        let c = spiral_side_id(n, i, w);

        // Smallest distance from center
        let side_min_val = c * (w - 1) + i - 1;
        let side_max_val = side_min_val + w - 1;
        let center = (side_min_val + side_max_val) / 2;
        let d = (n - center).abs();

        (d + (a - 1)) as u32
    }
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day03 {
            use ::*;

            #[test]
            fn helpers_spiral_id() {
                assert_eq!(1, spiral_id(1));
                for i in 2..(9 + 1) {
                    assert_eq!(2, spiral_id(i));
                }
                for i in 10..(25 + 1) {
                    assert_eq!(3, spiral_id(i));
                }
                for i in 26..(49 + 1) {
                    assert_eq!(4, spiral_id(i));
                }
            }

            #[test]
            fn helpers_spiral_width() {
                assert_eq!(1, spiral_width(1));
                assert_eq!(3, spiral_width(2));
                assert_eq!(5, spiral_width(3));
                assert_eq!(7, spiral_width(4));
            }

            #[test]
            fn helpers_spiral_first_val() {
                assert_eq!(1, spiral_first_val(1));
                assert_eq!(2, spiral_first_val(2));
                assert_eq!(10, spiral_first_val(3));
                assert_eq!(26, spiral_first_val(4));
            }

            #[test]
            fn helpers_spiral_side_id() {
                let calculate_spiral_side_id = |n| {
                    let id = spiral_id(n);
                    let first_val = spiral_first_val(id);
                    let width = spiral_width(id);
                    spiral_side_id(n, first_val, width)
                };

                // assert_eq!(1, calculate_spiral_side_id(1));
                assert_eq!(0, calculate_spiral_side_id(2));
                assert_eq!(0, calculate_spiral_side_id(3));
                assert_eq!(1, calculate_spiral_side_id(4));
                assert_eq!(1, calculate_spiral_side_id(5));
                assert_eq!(2, calculate_spiral_side_id(6));
                assert_eq!(2, calculate_spiral_side_id(7));
                assert_eq!(3, calculate_spiral_side_id(8));
                assert_eq!(3, calculate_spiral_side_id(9));

                assert_eq!(0, calculate_spiral_side_id(10));
                assert_eq!(0, calculate_spiral_side_id(11));
                assert_eq!(0, calculate_spiral_side_id(12));
                assert_eq!(0, calculate_spiral_side_id(13));
                assert_eq!(1, calculate_spiral_side_id(14));
                assert_eq!(1, calculate_spiral_side_id(15));
                assert_eq!(1, calculate_spiral_side_id(16));
                assert_eq!(1, calculate_spiral_side_id(17));
                assert_eq!(2, calculate_spiral_side_id(18));
                assert_eq!(2, calculate_spiral_side_id(19));
                assert_eq!(2, calculate_spiral_side_id(20));
                assert_eq!(2, calculate_spiral_side_id(21));
                assert_eq!(3, calculate_spiral_side_id(22));
                assert_eq!(3, calculate_spiral_side_id(23));
                assert_eq!(3, calculate_spiral_side_id(24));
                assert_eq!(3, calculate_spiral_side_id(25));

                assert_eq!(0, calculate_spiral_side_id(26));
                assert_eq!(0, calculate_spiral_side_id(27));
                assert_eq!(0, calculate_spiral_side_id(28));
                assert_eq!(0, calculate_spiral_side_id(29));
                assert_eq!(0, calculate_spiral_side_id(30));
                assert_eq!(0, calculate_spiral_side_id(31));
                assert_eq!(1, calculate_spiral_side_id(32));
                assert_eq!(1, calculate_spiral_side_id(33));
                assert_eq!(1, calculate_spiral_side_id(36));
                assert_eq!(1, calculate_spiral_side_id(37));
                assert_eq!(2, calculate_spiral_side_id(38));
                assert_eq!(2, calculate_spiral_side_id(39));
                assert_eq!(2, calculate_spiral_side_id(42));
                assert_eq!(2, calculate_spiral_side_id(43));
                assert_eq!(3, calculate_spiral_side_id(44));
                assert_eq!(3, calculate_spiral_side_id(48));
                assert_eq!(3, calculate_spiral_side_id(49));
            }

            #[test]
            fn example_01_1() {
                let input = 1;
                let expected = 0;
                let to_check = aoc_day03_part_1(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_02_12() {
                let input = 12;
                let expected = 3;
                let to_check = aoc_day03_part_1(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_03_23() {
                let input = 23;
                let expected = 2;
                let to_check = aoc_day03_part_1(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_04_1024() {
                let input = 1024;
                let expected = 31;
                let to_check = aoc_day03_part_1(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_custom_01_39() {
                let input = 39;
                let expected = 4;
                let to_check = aoc_day03_part_1(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_custom_02_corners() {
                let n_corners = [3, 5, 7, 9];
                let expected = 2;
                for input in n_corners.iter() {
                    let to_check = aoc_day03_part_1(*input);
                    assert_eq!(expected, to_check);
                }

                let n_corners = [13, 17, 21, 25];
                let expected = 4;
                for input in n_corners.iter() {
                    let to_check = aoc_day03_part_1(*input);
                    assert_eq!(expected, to_check);
                }
            }

            #[test]
            fn solution() {
                let puzzle_input = 325489;
                let expected = 552;
                let to_check = aoc_day03_part_1(puzzle_input);

                assert_eq!(expected, to_check);
            }
        }
    }
}
