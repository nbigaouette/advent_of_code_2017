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

use std::collections::HashMap;

struct MemoryLocation {
    // One-base (idx == 1 is the center)
    idx: u32,
}

impl MemoryLocation {
    fn new(idx: u32) -> MemoryLocation {
        MemoryLocation { idx: idx }
    }
    fn coordinates(&self) -> (i32, i32) {
        if self.idx == 1 {
            (0, 0)
        } else {
            let idx = self.idx as i32;

            // Spiral's id
            let a = self.spiral_id();
            // Square's width
            let w = self.spiral_width();
            // First number of the spiral
            let i = self.spiral_first_val();
            // Last number of the spiral
            // let j = (a * 2 - 1).pow(2);
            // Spiral's / square's side id
            let c = self.spiral_side_id();

            // Smallest distance from center
            let side_min_val = c * (w - 1) + i - 1;
            let side_max_val = side_min_val + w - 1;
            let center = (side_min_val + side_max_val) / 2;
            let d = idx - center;

            // Which quadrant?
            if c == 0 {
                (a - 1, d)
            } else if c == 1 {
                (-d, a - 1)
            } else if c == 2 {
                (-(a - 1), -d)
            } else if c == 3 {
                (d, -(a - 1))
            } else {
                unreachable!();
            }
        }
    }

    fn spiral_id(&self) -> i32 {
        (((self.idx as f32).sqrt() + 1.0) / 2.0).ceil() as i32
    }

    fn spiral_width(&self) -> i32 {
        let id = self.spiral_id();
        2 * (id - 1) + 1
    }

    fn spiral_first_val(&self) -> i32 {
        let id = self.spiral_id();
        ((id - 1) * 2 - 1).max(0).pow(2) + 1
    }

    fn spiral_side_id(&self) -> i32 {
        let first_val = self.spiral_first_val();
        let width = self.spiral_width();
        (self.idx as i32 - first_val) / (width - 1)
    }
}

fn populate_memory_location(n: u32, memory: &mut HashMap<(i32, i32), u32>) {
    let coordinates_shifts = [
        (1, 0),   // Right
        (1, 1),   // Up-Right
        (0, 1),   // Up
        (-1, 1),  // Up-Left
        (-1, 0),  // Left
        (-1, -1), // Down-Left
        (0, -1),  // Down
        (1, -1),  // Down-Right
    ];

    let coordinates = MemoryLocation::new(n).coordinates();
    let mut sum: u32 = 0;
    for shift in coordinates_shifts.iter() {
        let neighbors_coord = (coordinates.0 + shift.0, coordinates.1 + shift.1);
        let neighbors_value = memory.entry(neighbors_coord).or_insert(0);
        sum += *neighbors_value;
    }
    let location = memory.entry(coordinates).or_insert(0);
    *location += sum;
}

fn allocate_memory() -> HashMap<(i32, i32), u32> {
    // Initialize origin location to `1`
    [((0, 0), 1)].iter().cloned().collect()
}

pub fn get_memory_value_at(n: u32) -> u32 {
    let mut memory = allocate_memory();

    for ni in 2..n + 1 {
        populate_memory_location(ni, &mut memory);
    }

    let coordinates = MemoryLocation::new(n).coordinates();

    *memory.get(&coordinates).unwrap()
}

pub fn aoc_day03_part_2(n: u32) -> u32 {
    // Get the first value written that is larger than n
    let mut memory = allocate_memory();

    for ni in 2.. {
        populate_memory_location(ni, &mut memory);
        let coordinates = MemoryLocation::new(ni).coordinates();
        let value = *memory.get(&coordinates).unwrap();
        if value > n {
            return value;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day03_part_2 {
            use ::*;

            #[test]
            fn test_coordinates() {
                assert_eq!(MemoryLocation::new(1).coordinates(), (0, 0));

                assert_eq!(MemoryLocation::new(2).coordinates(), (1, 0));
                assert_eq!(MemoryLocation::new(3).coordinates(), (1, 1));
                assert_eq!(MemoryLocation::new(4).coordinates(), (0, 1));
                assert_eq!(MemoryLocation::new(5).coordinates(), (-1, 1));
                assert_eq!(MemoryLocation::new(6).coordinates(), (-1, 0));
                assert_eq!(MemoryLocation::new(7).coordinates(), (-1, -1));
                assert_eq!(MemoryLocation::new(8).coordinates(), (0, -1));
                assert_eq!(MemoryLocation::new(9).coordinates(), (1, -1));

                assert_eq!(MemoryLocation::new(10).coordinates(), (2, -1));
                assert_eq!(MemoryLocation::new(11).coordinates(), (2, 0));
                assert_eq!(MemoryLocation::new(12).coordinates(), (2, 1));
                assert_eq!(MemoryLocation::new(13).coordinates(), (2, 2));
                assert_eq!(MemoryLocation::new(14).coordinates(), (1, 2));
                assert_eq!(MemoryLocation::new(15).coordinates(), (0, 2));
                assert_eq!(MemoryLocation::new(16).coordinates(), (-1, 2));
                assert_eq!(MemoryLocation::new(17).coordinates(), (-2, 2));
                assert_eq!(MemoryLocation::new(18).coordinates(), (-2, 1));
                assert_eq!(MemoryLocation::new(19).coordinates(), (-2, 0));
                assert_eq!(MemoryLocation::new(20).coordinates(), (-2, -1));
                assert_eq!(MemoryLocation::new(21).coordinates(), (-2, -2));
                assert_eq!(MemoryLocation::new(22).coordinates(), (-1, -2));
                assert_eq!(MemoryLocation::new(23).coordinates(), (0, -2));
                assert_eq!(MemoryLocation::new(24).coordinates(), (1, -2));
                assert_eq!(MemoryLocation::new(25).coordinates(), (2, -2));
            }

            #[test]
            fn test_example_01() {
                let expecteds = [
                    1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330,
                    351, 362, 747, 806,
                ];
                for (n_minus_one, expected) in expecteds.iter().enumerate() {
                    let n = n_minus_one + 1;
                    let to_check = get_memory_value_at(n as u32);
                    assert_eq!(*expected, to_check);
                }
            }

            #[test]
            fn solution() {
                let puzzle_input = 325489;
                let expected = 330785;
                let to_check = aoc_day03_part_2(puzzle_input);

                assert_eq!(expected, to_check);
            }
        }
    }
}
