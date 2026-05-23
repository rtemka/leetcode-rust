// https://leetcode.com/problems/rotate-string/description
struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let s = String::from_iter(s.chars().chain(s.chars()));
        s.contains(&goal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_string() {
        assert!(Solution::rotate_string(
            "abcde".to_string(),
            "cdeab".to_string()
        ));
        assert!(!Solution::rotate_string(
            "abcde".to_string(),
            "abced".to_string()
        ));
        assert!(Solution::rotate_string(
            "xyza".to_string(),
            "xyza".to_string()
        ));
    }
}
