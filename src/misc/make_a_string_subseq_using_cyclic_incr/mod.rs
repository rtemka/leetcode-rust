// https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments/description
struct Solution;

impl Solution {
    // subsequence, not a substring. It doesn't have to be consecutive.
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        if str2.len() > str1.len() {
            return false;
        }
        let (b1, b2) = (str1.as_bytes(), str2.as_bytes());
        let (mut i, mut j) = (0, 0);
        while i < b1.len() && j < b2.len() {
            if (b2[j] == b'a' && b1[i] == b'z') || b2[j].wrapping_sub(b1[i]) < 2 {
                j += 1;
            }
            i += 1;
        }
        j == b2.len()
    }

    #[inline]
    fn _is_at_most_one_distance(str1: &str, str2: &str) -> bool {
        str1.chars().zip(str2.chars()).all(|(c1, c2)| {
            (c2 == 'a' && c1 == 'z') || (c2 as i16 - c1 as i16) == 0 || (c2 as i16 - c1 as i16) == 1
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_subsequence() {
        assert!(Solution::can_make_subsequence(
            "abc".to_string(),
            "ad".to_string()
        ));

        assert!(Solution::can_make_subsequence(
            "nbksekflejzgzhusd".to_string(),
            "osellfjzgais".to_string()
        ));

        assert!(!Solution::can_make_subsequence(
            "om".to_string(),
            "nm".to_string()
        ));
    }
}
