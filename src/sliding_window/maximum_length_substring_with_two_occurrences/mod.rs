// #3090
// https://leetcode.com/problems/maximum-length-substring-with-two-occurrences/description

struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let s = s.as_bytes();

        let mut longest = 0;
        let mut freq = [0_usize; 26];
        let mut lo = 0;

        for (hi, c) in s.iter().enumerate() {
            let cidx = (c - b'a') as usize;

            freq[cidx] += 1;

            while freq[cidx] > 2 {
                freq[(s[lo] - b'a') as usize] -= 1;
                lo += 1;
            }

            longest = longest.max(hi - lo + 1);
        }

        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_length_substring_with_two_occurrences() {
        assert_eq!(
            5,
            Solution::maximum_length_substring("adaddccdb".to_string())
        );

        assert_eq!(
            4,
            Solution::maximum_length_substring("bcbbbcba".to_string())
        );
    }
}
