use std::collections::HashMap;

// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/description
struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut count = 0;
        let mut m: HashMap<u8, usize> = HashMap::with_capacity(3);
        let s = s.as_bytes();
        let (mut lo, mut hi) = (0, 0);
        while hi < s.len() {
            if (b'a'..=b'c').contains(&s[hi]) {
                m.entry(s[hi]).and_modify(|cnt| *cnt += 1).or_insert(1);
            }
            if m.len() != m.capacity() {
                hi += 1;
                continue;
            }
            while lo < hi && m.len() == m.capacity() {
                count += s.len() - hi;
                let need_remove = if let Some(n) = m.get_mut(&s[lo]) {
                    *n -= 1;
                    *n == 0
                } else {
                    false
                };
                if need_remove {
                    m.remove(&s[lo]);
                }
                lo += 1;
            }
            hi += 1;
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_substrings_containing_all_three_chars() {
        assert_eq!(3, Solution::number_of_substrings("ababbbc".to_string()));
        assert_eq!(5, Solution::number_of_substrings("aabaabc".to_string()));
        assert_eq!(14, Solution::number_of_substrings("babcbbca".to_string()));
        assert_eq!(10, Solution::number_of_substrings("abcabc".to_string()));
        assert_eq!(1, Solution::number_of_substrings("cba".to_string()));
        assert_eq!(0, Solution::number_of_substrings("bba".to_string()));
    }
}
