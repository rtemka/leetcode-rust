use std::collections::{HashMap, HashSet};

// https://leetcode.com/problems/isomorphic-strings/description/
struct Solution;

impl Solution {
    //  1 <= s.length <= 5 * 104
    //  t.length == s.length
    //  s and t consist of any valid ascii character.

    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        let mut set = HashSet::new();
        for (left, right) in s.chars().zip(t.chars()) {
            match map.get(&left) {
                Some(&c) if c != right => return false,
                None => {
                    if set.contains(&right) {
                        return false;
                    }
                    set.insert(right);
                    map.insert(left, right);
                }
                _ => continue,
            };
        }
        true
    }

    pub fn is_isomorphic_ascii(s: String, t: String) -> bool {
        let mut left_ascii: [usize; 127] = [128; 127];
        let mut right_ascii: [usize; 127] = [128; 127];

        for (left, right) in s.chars().zip(t.chars()) {
            let (l, r) = (left as usize, right as usize);
            println!("left={left};right={right};l={l};r={r}");
            match left_ascii[l] {
                x if x == r => continue,
                x if x < 128 && x != r => return false,
                _ if right_ascii[r] == r => return false,
                _ => {
                    right_ascii[r] = r;
                    left_ascii[l] = r;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_isomorphic() {
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));
        assert!(Solution::is_isomorphic(
            "paper".to_string(),
            "title".to_string()
        ));
        assert!(!Solution::is_isomorphic(
            "badc".to_string(),
            "baba".to_string()
        ));
    }

    #[test]
    fn is_isomorphic_ascii() {
        assert!(Solution::is_isomorphic_ascii(
            "egg".to_string(),
            "add".to_string()
        ));
        assert!(!Solution::is_isomorphic_ascii(
            "foo".to_string(),
            "bar".to_string()
        ));
        assert!(Solution::is_isomorphic_ascii(
            "paper".to_string(),
            "title".to_string()
        ));
        assert!(!Solution::is_isomorphic_ascii(
            "badc".to_string(),
            "baba".to_string()
        ));
    }
}
