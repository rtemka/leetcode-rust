// https://leetcode.com/problems/valid-palindrome/description/
struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut i = s
            .chars()
            .filter(|&c| c.is_ascii_alphanumeric()).map(|c| c.to_ascii_lowercase());
        loop {
            match (i.next(), i.next_back()) {
                (Some(a), Some(b)) if a == b => continue,
                (_, None) | (None, _) => return true,
                (_, _) => return false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_palindrome() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
        assert!(!Solution::is_palindrome("race a car".to_string()));
        assert!(Solution::is_palindrome(" ".to_string()));
        assert!(Solution::is_palindrome("a".to_string()));
    }
}
