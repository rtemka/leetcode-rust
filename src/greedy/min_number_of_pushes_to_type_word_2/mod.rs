// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/description
struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut frequencies: [i32; 26] = [0; 26];
        for c in word.chars() {
            frequencies[c as usize - 0x61] += 1;
        }
        frequencies.sort_unstable();
        let mut count = 0;
        for (i, &freq) in frequencies.iter().rev().enumerate() {
            count = match i {
                ..=7 => count + freq,
                8..=15 => count + (freq * 2),
                16..=23 => count + (freq * 3),
                _ => count + (freq * 4),
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimun_pushes() {
        assert_eq!(5, Solution::minimum_pushes("abcde".to_string()));
        assert_eq!(12, Solution::minimum_pushes("xyzxyzxyzxyz".to_string()));
        assert_eq!(
            24,
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_string())
        );
    }
}
