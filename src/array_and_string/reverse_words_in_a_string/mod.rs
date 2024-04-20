// https://leetcode.com/problems/reverse-words-in-a-string/description/
struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut rs = String::new();
        for w in s.split_whitespace().rev() {
            rs.push_str(w);
            rs.push(' ');
        }
        if rs.len() > 0 {
            rs.truncate(rs.len() - 1);
        }
        rs
    }

    pub fn reverse_words_cheat(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_words_in_a_string() {
        assert_eq!(
            "blue is sky the".to_owned(),
            Solution::reverse_words("the sky is blue".to_string())
        );
        assert_eq!(
            "world hello".to_string(),
            Solution::reverse_words("  hello world  ".to_string())
        );
        assert_eq!(
            "example good a".to_string(),
            Solution::reverse_words("a good   example".to_string())
        );
    }
}
