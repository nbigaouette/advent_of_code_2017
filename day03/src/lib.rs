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

pub fn aoc_day03(n: u32) -> u32 {
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
                let to_check = aoc_day03(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_02_12() {
                let input = 12;
                let expected = 3;
                let to_check = aoc_day03(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_03_23() {
                let input = 23;
                let expected = 2;
                let to_check = aoc_day03(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_04_1024() {
                let input = 1024;
                let expected = 31;
                let to_check = aoc_day03(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn example_custom_01_39() {
                let input = 39;
                let expected = 4;
                let to_check = aoc_day03(input);

                assert_eq!(expected, to_check);
            }

            #[test]
            fn solution() {
                let puzzle_input = 325489;
                let expected = 552;
                let to_check = aoc_day03(puzzle_input);

                assert_eq!(expected, to_check);
            }
        }
    }
}
