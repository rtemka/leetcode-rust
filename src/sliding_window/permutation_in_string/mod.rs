// #567
// https://leetcode.com/problems/permutation-in-string/description/

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s2.len() < s1.len() {
            return false;
        }

        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());

        let mut expected_freq = [0_usize; 26];
        for c in s1 {
            expected_freq[(c - b'a') as usize] += 1;
        }

        let (mut lo, mut hi) = (0, 0);
        let mut freq = [0_usize; 26];

        while hi < s2.len() {
            freq[(s2[hi] - b'a') as usize] += 1;
            if hi - lo + 1 == s1.len() {
                if freq == expected_freq {
                    return true;
                }
                freq[(s2[lo] - b'a') as usize] -= 1;
                lo += 1;
            }
            hi += 1;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_in_string() {
        assert!(Solution::check_inclusion(
            "ab".to_string(),
            "eidbaooo".to_string()
        ));
    }
}
