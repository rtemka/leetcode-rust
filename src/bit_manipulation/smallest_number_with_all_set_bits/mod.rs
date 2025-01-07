// https://leetcode.com/problems/smallest-number-with-all-set-bits/description/
struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut r = 1;
        while r < n {
            r = r << 1 | 1;
            println!("{n:#034b}");
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smallest_number() {
        assert_eq!(7, Solution::smallest_number(5));
        assert_eq!(15, Solution::smallest_number(10));
        assert_eq!(3, Solution::smallest_number(3));
    }
}
