// https://leetcode.com/problems/palindrome-partitioning/description/
struct Solution;

impl Solution {
    pub fn partition(_s: String) -> Vec<Vec<String>> {
        vec![]
    }

    fn is_palindrome(s: &str) -> bool {
        if s.len() < 2 {
            return true;
        }
        s[0..s.len() / 2]
            .chars()
            .eq(s[s.len() / 2..s.len()].chars().rev())
    }
}

// "aaggaaggaa"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_partition() {
        assert!(Solution::is_palindrome("abba"));
        assert!(Solution::is_palindrome("aa"));
        assert_eq!(
            vec![
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
                vec!["aa".to_string(), "b".to_string()],
            ],
            Solution::partition("aab".to_string()),
        );
    }
}
