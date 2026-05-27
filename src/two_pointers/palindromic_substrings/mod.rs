// #647
// https://leetcode.com/problems/palindromic-substrings/description/

struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut count = 0;
        for i in 0..s.len() {
            for k in i + 1..=s.len() {
                if Self::is_palindrome(&s[i..k]) {
                    count += 1;
                }
            }
        }

        count
    }

    //https://www.youtube.com/watch?v=4RACzI5-du8
    pub fn count_substrings_better(s: String) -> i32 {
        let mut count = 0;
        for i in 0..s.len() {
            count += Self::count_palindromes(&s, i, i);
            count += Self::count_palindromes(&s, i, i + 1);
        }

        count
    }

    #[inline]
    fn is_palindrome(s: &str) -> bool {
        let mid = s.len() / 2;
        let odd = !s.len().is_multiple_of(2) as usize;

        s[..mid + odd].chars().eq(s[mid..].chars().rev())
    }

    #[inline]
    fn count_palindromes(s: &str, left: usize, right: usize) -> i32 {
        let left = s[..=left].chars().rev();
        let right = s[right..].chars();

        left.zip(right).take_while(|(x, y)| x == y).count() as i32

        // let mut result = 0;
        //
        // while let (Some(x), Some(y)) = (left.next_back(), right.next())
        //     && x == y
        // {
        //     result += 1;
        // }

        // result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindromic_substrings() {
        assert_eq!(3, Solution::count_substrings("abc".to_string()));
        assert_eq!(6, Solution::count_substrings("aaa".to_string()));

        assert_eq!(3, Solution::count_substrings_better("abc".to_string()));
        assert_eq!(6, Solution::count_substrings_better("aaa".to_string()));
    }

    #[test]
    fn palindromic_substrings_is_palindrome_helper() {
        assert!(Solution::is_palindrome("ababa"));
        assert!(Solution::is_palindrome("a"));
        assert!(Solution::is_palindrome("aa"));
        assert!(!Solution::is_palindrome("acc"));
        assert!(Solution::is_palindrome("abccba"));
    }
}
