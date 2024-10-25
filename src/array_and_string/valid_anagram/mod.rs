use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            map.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        for c in t.chars() {
            if let Some(cc) = map.get_mut(&c) {
                let _ = 1;
            }
        }
        false
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
    }
}
