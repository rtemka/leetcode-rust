// #438
// https://leetcode.com/problems/find-all-anagrams-in-a-string/description/
struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        // If len(p) exceeds len(s) then it impossible to form any anagram.
        if p.len() > s.len() {
            return vec![];
        }
        let (s, p) = (s.as_bytes(), p.as_bytes());
        // Setup 2 arrays to count frequencies of letters.
        let (mut expected, mut window) = ([0; 26], [0; 26]);
        // Populate 'expected' with the characters in string 'p'.
        for &c in p {
            expected[(c - b'a') as usize] += 1;
        }
        let mut result = Vec::new();
        let (mut lo, mut hi) = (0, 0);
        while hi < s.len() {
            // Add the character at the right pointer
            // to 'window' before sliding the window.
            window[(s[hi] - b'a') as usize] += 1;
            // If the window has reached the expected fixed length,
            // we advance the left pointer as well as the right pointer
            // to slide the window.
            if hi - lo + 1 == p.len() {
                if expected == window {
                    result.push(lo as i32);
                }
                // Remove the character at the left pointer from
                // 'window' before advancing the left pointer.
                window[(s[lo] - b'a') as usize] -= 1;
                lo += 1;
            }
            hi += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all_anagrams_in_a_string() {
        assert_eq!(
            vec![0, 6],
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string())
        )
    }
}
