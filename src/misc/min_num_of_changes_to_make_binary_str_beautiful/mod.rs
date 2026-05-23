// https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful/description
struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = 0;
        let mut i = 1;
        while i < s.len() {
            count += (s[i] != s[i - 1]) as i32;
            i += 2;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_changes() {
        assert_eq!(2, Solution::min_changes("1001".to_string()));
        assert_eq!(1, Solution::min_changes("10".to_string()));
        assert_eq!(0, Solution::min_changes("0000".to_string()));
        assert_eq!(2, Solution::min_changes("01010000".to_string()));
        assert_eq!(1, Solution::min_changes("01110000".to_string()));

        assert_eq!(3, Solution::min_changes("111101101110".to_string()));
    }
}
