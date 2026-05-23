use std::collections::HashMap;

// https://leetcode.com/problems/uncommon-words-from-two-sentences/description
struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut m: HashMap<&str, usize> = HashMap::with_capacity(s1.len());
        for word in s1.split_ascii_whitespace() {
            m.entry(word).and_modify(|c| *c += 1).or_insert(1);
        }
        for word in s2.split_ascii_whitespace() {
            m.entry(word).and_modify(|c| *c += 1).or_insert(1);
        }
        m.into_iter()
            .filter_map(|(word, count)| {
                if count == 1 {
                    Some(word.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uncommon_from_sentences() {
        let expected = vec!["sour".to_string(), "sweet".to_string()];
        let mut got = Solution::uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string(),
        );
        got.sort_unstable();
        assert_eq!(expected, got);
    }
}
