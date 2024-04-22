// https://leetcode.com/problems/reverse-words-in-a-string-iii/description/
struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut out = String::with_capacity(s.len());
        for w in s.split_ascii_whitespace() {
            out.push_str(&w.chars().rev().collect::<String>());
            out.push(' ');
        }
        if out.len() > 0 {
            out.truncate(out.len() - 1);
        }
        out
    }

    pub fn reverse_words_2(s: String) -> String {
        s.split_ascii_whitespace()
            .map(|word| word.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_words_in_a_string3() {
        assert_eq!(
            "s'teL ekat edoCteeL tsetnoc".to_string(),
            Solution::reverse_words_2("Let's take LeetCode contest".to_string())
        );
    }
}
