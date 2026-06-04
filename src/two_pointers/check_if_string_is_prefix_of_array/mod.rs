// #1961
// https://leetcode.com/problems/check-if-string-is-a-prefix-of-array/description/

struct Solution;

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let s = s.as_bytes();
        let mut i = 0;

        for word in words {
            if i == s.len() {
                return true;
            }

            let word = word.as_bytes();
            if i + word.len() > s.len() || s[i..i + word.len()] != *word {
                return false;
            }

            i += word.len();
        }

        i == s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_a_string_is_prefix_of_array() {
        assert!(Solution::is_prefix_string(
            "iloveleetcode".to_string(),
            vec![
                "i".to_string(),
                "love".to_string(),
                "leetcode".to_string(),
                // "apples".to_string()
            ]
        ));
    }
}
