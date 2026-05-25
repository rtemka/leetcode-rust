// #524
// https://leetcode.com/problems/longest-word-in-dictionary-through-deleting/description/

struct Solution;

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut longest: String = String::from("");

        for word in dictionary.into_iter() {
            // If candidate word is longer than `s` it is impossible to form it from `s`.
            // If candidate word is shorter than current longest word then don't bother checking it.
            if word.len() > s.len() || word.len() < longest.len() {
                continue;
            }

            // This is the Rust implementation of 2 pointers -> left and right.
            // Peekable iterator allows us to lookup next values in the iterator without advancing
            // them in the same time.
            let (mut left, mut right) = (s.chars().peekable(), word.chars().peekable());
            while let (Some(&a), Some(&b)) = (left.peek(), right.peek()) {
                // If `s` has target char then advance both iterators else advance only `s`
                // iterator.
                if a == b {
                    let (_, _) = (left.next(), right.next());
                } else {
                    let _ = left.next();
                }
            }

            // If candidate word was fully processed than check if it is longer than current max.
            // If candidate and current max has same length then check which one is smaller
            // lexicographicaly.
            if right.next().is_none() {
                longest = if word.len() > longest.len() {
                    word
                } else {
                    longest.min(word)
                };
            }
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_word_in_dict_thru_deleting() {
        assert_eq!(
            "apple".to_string(),
            Solution::find_longest_word(
                "abpcplea".to_string(),
                vec![
                    "ale".to_string(),
                    "apple".to_string(),
                    "monkey".to_string(),
                    "plea".to_string()
                ]
            )
        );

        assert_eq!(
            "a".to_string(),
            Solution::find_longest_word(
                "abpcplea".to_string(),
                vec!["a".to_string(), "b".to_string(), "c".to_string(),]
            )
        );
    }
}
