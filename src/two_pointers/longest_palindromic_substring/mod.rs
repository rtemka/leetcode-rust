// #5
// https://leetcode.com/problems/longest-palindromic-substring/description/

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut max_palindrome: &str = "";

        for i in 0..s.len() {
            // Find palindromes with odd length
            let l = Self::count_length_of_palindrome(&s, i, i);

            let candidate = &s[i + 1 - l..i + l];
            if candidate.len() > max_palindrome.len() {
                max_palindrome = candidate;
            }

            // Find palindromes with even length
            let l = Self::count_length_of_palindrome(&s, i, i + 1);

            let candidate = &s[i + 1 - l..i + 1 + l];
            if candidate.len() > max_palindrome.len() {
                max_palindrome = candidate;
            }
        }

        max_palindrome.to_owned()
    }

    #[inline]
    fn count_length_of_palindrome(s: &str, lo: usize, hi: usize) -> usize {
        s[..=lo]
            .chars()
            .rev()
            .zip(s[hi..].chars())
            .take_while(|(l, r)| l == r)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_palindromic_substring() {
        assert_eq!("b", Solution::longest_palindrome("b".to_string()));
        assert_eq!("bab", Solution::longest_palindrome("babad".to_string()));
        assert_eq!("bb", Solution::longest_palindrome("cbbd".to_string()));
        assert_eq!("bb", Solution::longest_palindrome("cbbdzt".to_string()));
        assert_eq!("ccc", Solution::longest_palindrome("ccc".to_string()));
    }
}
