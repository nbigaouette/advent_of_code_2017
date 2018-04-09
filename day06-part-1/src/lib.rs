//! # Day 6: Memory Reallocation
//!
//! ## Part One
//!
//! A debugger program here is having an issue: it is trying to repair a memory reallocation routine, but it keeps getting stuck in an infinite loop.
//!
//! In this area, there are sixteen memory banks; each memory bank can hold any number of _blocks_. The goal of the reallocation routine is to balance the blocks between the memory banks.
//!
//! The reallocation routine operates in cycles. In each cycle, it finds the memory bank with the most blocks (ties won by the lowest-numbered memory bank) and redistributes those blocks among the banks. To do this, it removes all of the blocks from the selected bank, then moves to the next (by index) memory bank and inserts one of the blocks. It continues doing this until it runs out of blocks; if it reaches the last memory bank, it wraps around to the first one.
//!
//! The debugger would like to know how many redistributions can be done before a blocks-in-banks configuration is produced that _has been seen before_.
//!
//! For example, imagine a scenario with only four memory banks:
//!
//! *   The banks start with `0`, `2`, `7`, and `0` blocks. The third bank has the most blocks, so it is chosen for redistribution.
//! *   Starting with the next bank (the fourth bank) and then continuing to the first bank, the second bank, and so on, the `7` blocks are spread out over the memory banks. The fourth, first, and second banks get two blocks each, and the third bank gets one back. The final result looks like this: `2 4 1 2`.
//! *   Next, the second bank is chosen because it contains the most blocks (four). Because there are four memory banks, each gets one block. The result is: `3 1 2 3`.
//! *   Now, there is a tie between the first and fourth memory banks, both of which have three blocks. The first bank wins the tie, and its three blocks are distributed evenly over the other three banks, leaving it with none: `0 2 3 4`.
//! *   The fourth bank is chosen, and its four blocks are distributed such that each of the four banks receives one: `1 3 4 1`.
//! *   The third bank is chosen, and the same thing happens: `2 4 1 2`.
//!
//! At this point, we've reached a state we've seen before: `2 4 1 2` was already seen. The infinite loop is detected after the fifth block redistribution cycle, and so the answer in this example is `5`.
//!
//! Given the initial block counts in your puzzle input, _how many redistribution cycles_ must be completed before a configuration is produced that has been seen before?
//!
//! ## Part Two
//!
//! Out of curiosity, the debugger would also like to know the size of the loop: starting from a state that has already been seen, how many block redistribution cycles must be performed before that same state is seen again?
//!
//! In the example above, `2 4 1 2` is seen again after four cycles, and so the answer in that example would be `4`.
//!
//! _How many cycles_ are in the infinite loop that arises from the configuration in your puzzle input?

extern crate crc;

use std::collections::HashSet;

fn distribute_once(banks: &mut [u32]) {
    // Find bank with maximum block
    let (max_val, max_idx): (u32, u32) =
        banks.iter().enumerate().fold((0, 0), |acc, (idx, val)| {
            if *val > acc.0 {
                (*val, idx as u32)
            } else {
                acc
            }
        });

    banks[max_idx as usize] = 0;

    let banks_len = banks.len() as u32;
    // All banks will receive this increment
    let to_increment_all = max_val / banks_len;
    // A subset of banks, starting at `max_idx+1` will receive this increment
    let to_increment_few = max_val % banks_len;
    // The size of the subset's left part
    let subset_size_left = if to_increment_few > (banks_len - max_idx - 1) {
        (to_increment_few - (banks_len - max_idx - 1)) as usize
    } else {
        0
    };
    // The size of the subset's left right
    let subset_size_right = to_increment_few as usize - subset_size_left;

    // Increment all banks
    banks.iter_mut().for_each(|val| *val += to_increment_all);

    // Increment banks starting at `max_idx` until the end
    banks
        .iter_mut()
        .skip((max_idx + 1) as usize)
        .take(subset_size_right)
        .for_each(|val| *val += 1);

    // Increment banks starting at 0
    if to_increment_few > 0 {
        banks
            .iter_mut()
            .take(subset_size_left)
            .for_each(|val| *val += 1);
    }
}

/// Re-interpret the slice of `&[T]` as a slice of `&[u8]`
fn as_u8_slice<T>(v: &[T]) -> &[u8] {
    let element_size = std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * element_size) }
}

fn reallocation(banks: &mut [u32]) -> u32 {
    // A HashSet is used to detect if a banks configuration was already seen
    // and stop the redistribution.
    let mut banks_seen = HashSet::new();

    let mut count = 0;

    // Since the slice cannot be used as the key to the HashSet,
    // a CRC32 checksum of the slice is calculated instead and used
    // as the HashSet key.
    // If the `replace()` method returns None, the
    // checksum was never seen and the looping thus continue.
    // NOTE: Since the crc function takes a `&[u8]`, the banks' `&[u32]` is
    //       converted using `as_u8_slice()`.
    while let None = banks_seen.replace(crc::crc32::checksum_ieee(as_u8_slice(banks))) {
        distribute_once(banks);
        count += 1;
    }

    count
}

pub fn aoc_day06_part_1(banks: &str) -> u32 {
    let mut banks: Vec<u32> = banks
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    reallocation(&mut banks)
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day06_part_1 {
            use ::*;

            #[test]
            fn example_01_step1_0270() {
                let mut banks = [0, 2, 7, 0];
                let expected = [2, 4, 1, 2];
                distribute_once(&mut banks);

                assert_eq!(expected, banks);
            }

            #[test]
            fn example_01_step2_2412() {
                let mut banks = [2, 4, 1, 2];
                let expected = [3, 1, 2, 3];
                distribute_once(&mut banks);

                assert_eq!(expected, banks);
            }

            #[test]
            fn example_01_step3_3123() {
                let mut banks = [3, 1, 2, 3];
                let expected = [0, 2, 3, 4];
                distribute_once(&mut banks);

                assert_eq!(expected, banks);
            }

            #[test]
            fn example_01_step4_0234() {
                let mut banks = [0, 2, 3, 4];
                let expected = [1, 3, 4, 1];
                distribute_once(&mut banks);

                assert_eq!(expected, banks);
            }

            #[test]
            fn example_01_step5_1341() {
                let mut banks = [1, 3, 4, 1];
                let expected = [2, 4, 1, 2];
                distribute_once(&mut banks);

                assert_eq!(expected, banks);
            }

            #[test]
            fn example_01_0270() {
                let input = "0	2	7	0";
                let expected = 5;
                let to_check = aoc_day06_part_1(&input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution() {
                const PUZZLE_INPUT: &'static str = include_str!("../input.txt");
                let expected = 6681;
                let to_check = aoc_day06_part_1(PUZZLE_INPUT);

                assert_eq!(expected, to_check);
            }
        }
    }
}
