use std::collections::HashMap;

// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set: HashMap<char, i32> = HashMap::new();
        let mut max = 0;
        let mut lo = -1;
        for (hi, ch) in s.chars().enumerate() {
            // println!("cur_char:{}\tset:{:?}\tlo:{}-hi{}", ch, set, lo, hi);
            if let Some(i) = set.insert(ch, hi as i32) {
                // println!("hit the char={}:i={}:lo={}", ch, i, lo);
                lo = lo.max(i);
                // println!("lo={}", lo);
            }
            max = max.max(hi as i32 - lo);
        }
        max
    }

    pub fn length_of_longest_substring_magic(s: String) -> i32 {
        let mut max_len: usize = 0;

        // [1] longest substring is the one with the largest
        //     difference of positions of repeated characters;
        //     thus, we should create a storage for such positions
        let mut pos: [usize; 128] = [0; 128];

        // [2] while iterating through the string (i.e., moving
        //     the end of the sliding window), we should also
        //     update the start of the window
        let mut start: usize = 0;

        for (end, ch) in s.chars().enumerate() {
            // [3] get the position for the start of sliding window
            //     with no other occurences of 'ch' in it
            start = start.max(pos[ch as usize]);

            // [4] update maximum length
            max_len = max_len.max(end - start + 1);

            // [5] set the position to be used in [3] on next iterations
            pos[ch as usize] = end + 1;
        }

        return max_len as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_of_longest_substring() {
        assert_eq!(
            8,
            Solution::length_of_longest_substring("aZbZcjtyaw".to_string()),
        );
        // assert_eq!(
        //     3,
        //     Solution::length_of_longest_substring("abcabcbb".to_string()),
        // );
        // assert_eq!(
        //     1,
        //     Solution::length_of_longest_substring("bbbbbbbbbbb".to_string()),
        // );
        // assert_eq!(
        //     3,
        //     Solution::length_of_longest_substring("pwwkew".to_string()),
        // );
        // assert_eq!(
        //     7,
        //     Solution::length_of_longest_substring("abcdifg".to_string()),
        // );
        // assert_eq!(2, Solution::length_of_longest_substring("au".to_string()),);
    }
}
