// https://leetcode.com/problems/minimum-window-substring/description/
struct Solution;

use std::cmp::Ordering;

const OFFSET: usize = 65;

#[derive(PartialEq, Debug)]
struct Letters {
    map: [usize; 123 - OFFSET],
    len: usize,
}

impl Letters {
    #[inline]
    fn new() -> Self {
        Self {
            map: [0; 123 - OFFSET],
            len: 0,
        }
    }

    #[inline]
    fn contains(&self, letter: u8) -> bool {
        self.map[letter as usize - OFFSET] > 0
    }

    #[inline]
    fn clear(&mut self) {
        self.map.fill(0);
    }

    #[inline]
    fn get(&self, letter: u8) -> usize {
        self.map[letter as usize - OFFSET]
    }

    #[inline]
    fn incr(&mut self, letter: u8) {
        self.map[letter as usize - OFFSET] += 1;
        self.len += 1;
    }

    #[inline]
    fn decr(&mut self, letter: u8) {
        if self.map[letter as usize - OFFSET] == 0 {
            return;
        }
        self.map[letter as usize - OFFSET] -= 1;
        self.len -= 1;
    }
}
//
// impl From<&[u8]> for Letters {
//     fn from(s: &[u8]) -> Self {
//         let mut letters = Letters::new();
//         for &c in s {
//             letters.incr(c);
//         }
//         letters
//     }
// }

impl PartialOrd for Letters {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.len < other.len {
            return Some(Ordering::Less);
        }
        let mut i = 0;
        let mut greater_count = 0;
        while i < self.map.len() {
            if self.map[i] < other.map[i] {
                return Some(Ordering::Less);
            } else if self.map[i] > other.map[i] {
                greater_count += 1;
            }
            i += 1;
        }
        if greater_count == 0 {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        // Constraints:
        //
        //     m == s.length
        //     n == t.length
        //     1 <= m, n <= 10^5
        //     s and t consist of uppercase and lowercase English letters.

        // if t.len > s.len no window is possible.
        if t.len() > s.len() {
            return "".to_string();
        }

        // Letters is an kind of hashmap with
        // only uppercase and lowercase Engilish letters.
        // It allowes for quick check and comparison of
        // two instances of the Letters.
        let mut ms = Letters::new();
        let mut mt = Letters::new();
        let sb = s.as_bytes();
        // Put all letters from t to map.
        for &c in t.as_bytes() {
            mt.incr(c);
        }

        let (mut lo, mut hi) = (0, 0);
        // we'll keep only two pointers that
        // represents the current minimum window.
        let mut min_range = (0, 0);
        while hi < sb.len() {
            // advance the hi pointer until we find all the letters from t.
            while hi < sb.len() && ms < mt {
                if mt.contains(sb[hi]) {
                    ms.incr(sb[hi]);
                }
                hi += 1;
            }
            // if we pass thru entire string and still doesn't find all letters: out.
            if hi == sb.len() && ms < mt {
                break;
            }
            loop {
                // advance the lo pointer until all letters of t holds in our letters map.
                ms.decr(sb[lo]);
                if ms >= mt {
                    lo += 1;
                } else {
                    break;
                }
            }
            // compare min range and current
            min_range = if min_range == (0, 0) || min_range.1 - min_range.0 > hi - lo {
                (lo, hi)
            } else {
                min_range
            };
            // println!("{:?}{lo},{hi}", ms);
            // println!("candidate={:?}", &s[lo..hi]);
            // println!("min={:?}", &s[min_range.0..min_range.1]);
            lo += 1;
        }
        // slice the minimum substring window from s.
        s[min_range.0..min_range.1].into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_window_substring() {
        assert_eq!(
            "ab".to_string(),
            Solution::min_window("bdab".to_string(), "ab".to_string())
        );
        assert_eq!(
            "abbbbbcdd".to_string(),
            Solution::min_window("aaaaaaaaaaaabbbbbcdd".to_string(), "abcdd".to_string())
        );
        assert_eq!(
            "".to_string(),
            Solution::min_window("a".to_string(), "b".to_string())
        );
        assert_eq!(
            "a".to_string(),
            Solution::min_window("a".to_string(), "a".to_string())
        );
        assert_eq!(
            "a".to_string(),
            Solution::min_window("aaaaaaaaaaaaa".to_string(), "a".to_string())
        );
        assert_eq!(
            "".to_string(),
            Solution::min_window("a".to_string(), "aa".to_string())
        );
        assert_eq!(
            "ABBz".to_string(),
            Solution::min_window("AAAAABBz".to_string(), "ABz".to_string())
        );
        assert_eq!(
            "ABBBBBBBC".to_string(),
            Solution::min_window("AAAAABBBBBBBCCCCCCC".to_string(), "ABC".to_string())
        );
        assert_eq!(
            "BANC".to_string(),
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
        );
    }
}
