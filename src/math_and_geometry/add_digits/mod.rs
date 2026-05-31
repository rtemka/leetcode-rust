// #258
// https://leetcode.com/problems/add-digits/description/
// Digital Root

struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        // https://codeforces.com/blog/entry/18286
        1 + ((num - 1) % 9)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_digits() {
        assert_eq!(2, Solution::add_digits(38));
        assert_eq!(0, Solution::add_digits(0));
        assert_eq!(9, Solution::add_digits(18));
        assert_eq!(9, Solution::add_digits(27));
    }
}
