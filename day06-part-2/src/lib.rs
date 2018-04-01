extern crate crc;

use std::collections::HashMap;

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

fn reallocation(banks: &mut [u32]) -> (u32, u32) {
    // A HashMap is used to detect if a banks configuration was already seen
    // and stop the redistribution.
    let mut banks_seen = HashMap::new();

    let mut count = 0;
    let mut first_seen_at = 0;

    // Since the slice cannot be used as the key to the HashMap,
    // a CRC32 checksum of the slice is calculated instead and used
    // as the HashMap key.
    // If the `replace()` method returns None, the
    // checksum was never seen and the looping thus continue.
    // NOTE: Since the crc function takes a `&[u8]`, the banks' `&[u32]` is
    //       converted using `as_u8_slice()`.
    for i in 0.. {
        let key = crc::crc32::checksum_ieee(as_u8_slice(banks));
        let entry = banks_seen.entry(key).or_insert((i, 0));
        (*entry).1 += 1;
        if (*entry).1 > 1 {
            first_seen_at = (*entry).0;
            break;
        }
        distribute_once(banks);
        count += 1;
    }

    (count, first_seen_at)
}

pub fn aoc_day06_part_2(banks: &str) -> (u32, u32, Vec<u32>) {
    let mut banks: Vec<u32> = banks
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let (count, first_seen_at) = reallocation(&mut banks);
    (count, count - first_seen_at, banks)
}

#[cfg(test)]
mod tests {
    mod aoc2017 {
        mod day06_part_2 {
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
            fn example_01_2412() {
                let input = "0	2	7	0";
                let (expected_count, expected_first_seen_at, expected_banks) =
                    (5, 4, &[2, 4, 1, 2]);
                let (to_check_count, to_check_first_seen_at, to_check_vec) =
                    aoc_day06_part_2(&input);

                assert_eq!(to_check_count, expected_count);
                assert_eq!(to_check_first_seen_at, expected_first_seen_at);
                assert_eq!(to_check_vec, expected_banks);
            }

            #[test]
            fn solution() {
                const PUZZLE_INPUT: &'static str = include_str!("../input.txt");
                let (expected_count, expected_first_seen_at, expected_banks) = (
                    6681,
                    2392,
                    &[0, 14, 13, 12, 11, 10, 8, 8, 6, 6, 5, 3, 3, 2, 1, 10],
                );
                let (to_check_count, to_check_first_seen_at, to_check_vec) =
                    aoc_day06_part_2(PUZZLE_INPUT);

                assert_eq!(to_check_count, expected_count);
                assert_eq!(to_check_first_seen_at, expected_first_seen_at);
                assert_eq!(to_check_vec, expected_banks);
            }
        }
    }
}
