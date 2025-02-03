use std::collections::HashMap;

struct Solution {}

// https://leetcode.com/problems/ransom-note/
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if magazine.len() < ransom_note.len() {
            return false;
        }
        let mut m: HashMap<char, i32> = HashMap::new();
        for char in magazine.chars() {
            m.entry(char).and_modify(|v| *v += 1).or_insert(1);
        }
        for char in ransom_note.chars() {
            match m.get_mut(&char) {
                Some(count) if *count > 0 => *count -= 1,
                _ => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ransom_note() {
        assert!(
            Solution::can_construct("aa".to_string(), "aab".to_string())
        );
        assert!(
            Solution::can_construct("art".to_string(), "somethingaaaarrrrtttt".to_string())
        );
        assert!(
            !Solution::can_construct("xxxmmm".to_string(), "mmmassstt".to_string())
        );
    }
}
