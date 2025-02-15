// https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c/description
struct Solution;

impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut count = 0;
        for _ in 1..32 {
            // println!("a: {:034b}", a);
            // println!("b: {:034b}", b);
            // println!("c: {:034b}\n", c);
            count += match (c & 1, a & 1, b & 1) {
                (1, 0, 0) => 1,
                (0, 1, 0) => 1,
                (0, 0, 1) => 1,
                (0, 1, 1) => 2,
                _ => 0,
            };
            (a, b, c) = (a >> 1, b >> 1, c >> 1);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_flips_to_make_a_or_b_eq_c() {
        assert_eq!(3, Solution::min_flips(2, 6, 5));
        assert_eq!(1, Solution::min_flips(4, 2, 7));
        assert_eq!(0, Solution::min_flips(1, 2, 3));
        assert_eq!(29, Solution::min_flips(2, 6, i32::MAX));
    }
}
