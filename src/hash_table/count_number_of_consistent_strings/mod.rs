use std::collections::HashSet;

// https://leetcode.com/problems/count-the-number-of-consistent-strings/description/
struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let m: HashSet<char> = allowed.chars().collect();
        words
            .iter()
            .fold(0, |acc, w| acc + (w.chars().all(|c| m.contains(&c))) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_consistent_strings() {
        assert_eq!(
            2,
            Solution::count_consistent_strings(
                "ab".to_string(),
                vec![
                    "ad".to_string(),
                    "bd".to_string(),
                    "aaab".to_string(),
                    "baa".to_string(),
                    "badab".to_string()
                ]
            )
        );
        assert_eq!(
            7,
            Solution::count_consistent_strings(
                "abc".to_string(),
                vec![
                    String::from("a"),
                    "b".to_string(),
                    "c".to_owned(),
                    "ab".to_string(),
                    "ac".to_string(),
                    "bc".to_string(),
                    "abc".to_string()
                ]
            )
        );
    }
}
