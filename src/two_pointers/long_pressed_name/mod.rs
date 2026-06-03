// #925
// https://leetcode.com/problems/long-pressed-name/description/

struct Solution;

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        if typed.len() < name.len() {
            return false;
        }

        let (mut left, mut right) = (0, 0);
        let (name, typed) = (name.as_bytes(), typed.as_bytes());

        while left < name.len() {
            let mut l = left;
            while l < name.len() && name[l] == name[left] {
                l += 1;
            }

            let mut r = right;
            while r < typed.len() && typed[r] == name[left] {
                r += 1;
            }

            if r - right < l - left {
                return false;
            }

            (left, right) = (l, r);
        }

        right == typed.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_pressed_name() {
        assert!(Solution::is_long_pressed_name(
            "alex".to_string(),
            "aaleex".to_string()
        ));

        assert!(!Solution::is_long_pressed_name(
            "alex".to_string(),
            "aaleexa".to_string()
        ));

        assert!(!Solution::is_long_pressed_name(
            "saeed".to_string(),
            "ssaaedd".to_string()
        ));

        assert!(Solution::is_long_pressed_name(
            "saeee".to_string(),
            "ssaaeee".to_string()
        ));

        assert!(!Solution::is_long_pressed_name(
            "a".to_string(),
            "s".to_string()
        ));

        assert!(Solution::is_long_pressed_name(
            "a".to_string(),
            "a".to_string()
        ));

        assert!(Solution::is_long_pressed_name(
            "a".to_string(),
            "aaaaaaaaaaaaaa".to_string()
        ));
    }
}
