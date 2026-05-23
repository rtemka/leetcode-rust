// https://leetcode.com/problems/move-pieces-to-obtain-a-string/description
struct Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let (sb, tb) = (start.as_bytes(), target.as_bytes());
        let (mut i, mut j) = (0, 0);
        loop {
            while i < sb.len() && sb[i] == b'_' {
                i += 1;
            }
            while j < tb.len() && tb[j] == b'_' {
                j += 1;
            }
            if i == sb.len() && j == tb.len() {
                break;
            }
            // The sequence must be the same in `start` and `target`
            // In the `target`, no `L` should be righter than the corresponding `L` in `start`
            // and no `R` should be lefter than the corresponding `R` in `start`
            if i == sb.len()
                || j == tb.len()
                || sb[i] != tb[j]
                || (sb[i] == b'L' && i < j)
                || (sb[i] == b'R' && i > j)
            {
                return false;
            }
            (i, j) = (i + 1, j + 1);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_change() {
        assert!(Solution::can_change(
            "_L__R__R_".to_string(),
            "L______RR".to_string()
        ));

        assert!(!Solution::can_change(
            "L__R__R_L".to_string(),
            "L______RR".to_string()
        ));

        assert!(!Solution::can_change(
            "R__R_".to_string(),
            "_RR__".to_string()
        ));
    }
}
