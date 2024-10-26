use std::collections::HashMap;

// https://leetcode.com/problems/valid-anagram/description/
struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        const OFFSET: usize = 97;
        let mut lower_ascii: [isize; 123 - OFFSET] = [0; 123 - OFFSET];
        for (c_left, c_right) in s.chars().zip(t.chars()) {
            lower_ascii[c_left as usize - OFFSET] += 1;
            lower_ascii[c_right as usize - OFFSET] -= 1;
        }
        lower_ascii.iter().all(|&i| i == 0)
    }

    pub fn is_anagram_hash(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            map.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        for c in t.chars() {
            if let Some(cc) = map.get_mut(&c) {
                if *cc == 1 {
                    map.remove(&c);
                } else {
                    *cc -= 1;
                }
            } else {
                return false;
            }
        }
        map.len() == 0
    }

    pub fn is_anagram_sort(s: String, t: String) -> bool {
        let mut si = s.into_bytes();
        si.sort_unstable();
        let mut ti = t.into_bytes();
        ti.sort_unstable();
        si == ti
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_anagram() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));

        assert!(!Solution::is_anagram(
            "superdupert".to_string(),
            "superdupers".to_string()
        ));

        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));

        assert!(Solution::is_anagram_sort(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }
}
