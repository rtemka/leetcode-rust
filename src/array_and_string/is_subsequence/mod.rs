// https://leetcode.com/problems/is-subsequence/description
struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let (sb, tb) = (s.into_bytes(), t.into_bytes());
        let mut sp = 0;
        for &c in tb.iter() {
            if c == sb[sp] {
                sp += 1;
                if sp == sb.len() {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_subsequence() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(!Solution::is_subsequence(
            "axc".to_string(),
            "ahbgdc".to_string()
        ));
        assert!(Solution::is_subsequence(
            "leeeeetcode".to_string(),
            "leeeeeetcode".to_string()
        ));
    }
}
