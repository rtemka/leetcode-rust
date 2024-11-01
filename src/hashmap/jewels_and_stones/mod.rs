use std::collections::HashSet;

// https://leetcode.com/problems/jewels-and-stones/description/
struct Solution;

const OFFSET: usize = 65;

impl Solution {
    pub fn num_jewels_in_stones2(jewels: String, stones: String) -> i32 {
        let mut map: [u8; 123 - OFFSET] = [0; 123 - OFFSET];
        for c in jewels.chars() {
            map[c as usize - OFFSET] = c as u8;
        }
        stones.chars().fold(0, |acc, c| {
            acc + (map[c as usize - OFFSET] == c as u8) as i32
        })
    }

    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let set: HashSet<char> = jewels.chars().collect();
        stones
            .chars()
            .fold(0, |acc, c| acc + (set.contains(&c)) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_jewels_in_stones() {
        assert_eq!(
            3,
            Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        );
        assert_eq!(
            6,
            Solution::num_jewels_in_stones("aAzyxQ".to_string(), "tuyQaioprsrzaQ".to_string()),
        );
    }
}
