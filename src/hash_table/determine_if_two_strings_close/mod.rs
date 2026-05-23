// https://leetcode.com/problems/determine-if-two-strings-are-close/description
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        let mut letters: HashMap<char, usize> = HashMap::new();
        for c in word1.chars() {
            letters
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut freq1: HashMap<usize, usize> = HashMap::with_capacity(letters.len());
        for v in letters.values_mut() {
            freq1.entry(*v).and_modify(|count| *count += 1).or_insert(1);
            *v = 0;
        }
        // check that word2 has same letters.
        for c in word2.chars() {
            if let Some(count) = letters.get_mut(&c) {
                *count += 1;
            } else {
                return false;
            }
        }
        let mut freq2 = HashMap::with_capacity(letters.len());
        for v in letters.values() {
            freq2.entry(*v).and_modify(|count| *count += 1).or_insert(1);
        }
        freq1 == freq2
    }

    pub fn close_strings_better(word1: String, word2: String) -> bool {
        // create mappings from characters to their
        // frequencies in word1 and word2
        let (mut map1, mut map2) = ([0; 26], [0; 26]);
        for c in word1.chars() {
            map1[c as usize - 0x61] += 1;
        }
        for c in word2.chars() {
            map2[c as usize - 0x61] += 1;
        }

        // if there is character that one word contains
        // but the other doesn't, then they can't be 'close'
        for i in 0..26 {
            if (map1[i] == 0 && map2[i] != 0) || (map1[i] != 0 && map2[i] == 0) {
                return false;
            }
        }

        // sort the maps such that we can make sure the amount of characters
        // with a given frequency match
        //
        // since maps are of size 26, this would be 2*O(26log26) === O(1)
        map1.sort();
        map2.sort();

        map1 == map2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_strings() {
        assert!(Solution::close_strings(
            "abc".to_string(),
            "bca".to_string()
        ));
        assert!(!Solution::close_strings("a".to_string(), "aa".to_string()));
        assert!(Solution::close_strings(
            "cabbba".to_string(),
            "abbccc".to_string()
        ));
        assert!(!Solution::close_strings(
            "cabbba".to_string(),
            "abbbbc".to_string()
        ));
        assert!(!Solution::close_strings(
            "aaabbbbccddeeeeefffff".to_string(),
            "aaaaabbcccdddeeeeffff".to_string()
        ));
        assert!(Solution::close_strings("a".to_string(), "a".to_string()));
    }
}
