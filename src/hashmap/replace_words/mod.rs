struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut res = String::with_capacity(sentence.len());
        let set: HashSet<&str> = dictionary.iter().map(AsRef::as_ref).collect();
        for word in sentence.split_ascii_whitespace() {
            let l = res.len();
            let mut i = 0;
            while i < word.len() {
                if let Some(root) = set.get(&word[..=i]) {
                    res.push_str(root);
                    res.push(' ');
                    break;
                }
                i += 1;
            }
            // no match
            if l == res.len() {
                res.push_str(word);
                res.push(' ');
            }
        }
        // rm last ' '
        if res.len() > 0 {
            res.truncate(res.len() - 1);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_words() {
        assert_eq!(
            "the cat was rat by the bat".to_string(),
            Solution::replace_words(
                vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
                "the cattle was rattled by the battery".to_string()
            )
        );
        assert_eq!(
            "a a b c".to_string(),
            Solution::replace_words(
                vec!["a".to_string(), "b".to_string(), "c".to_string()],
                "aadsfasf absbs bbab cadsfafs".to_string()
            )
        );
    }
}
