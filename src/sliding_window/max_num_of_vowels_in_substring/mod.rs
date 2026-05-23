// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/description
struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let bs = s.into_bytes();
        let mut max = 0;
        for i in 0..k {
            if Self::is_vowel(bs[i]) {
                max += 1;
            }
        }
        let mut cur = max;
        for i in k..bs.len() {
            cur += (Self::is_vowel(bs[i])) as i32;
            cur -= (Self::is_vowel(bs[i - k])) as i32;
            max = max.max(cur);
        }
        max
    }

    fn is_vowel(c: u8) -> bool {
        match c {
            b'a' | b'e' | b'i' | b'o' | b'u' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_vowels() {
        assert_eq!(3, Solution::max_vowels("abciiidef".to_string(), 3));
        assert_eq!(2, Solution::max_vowels("aeiou".to_string(), 2));
        assert_eq!(2, Solution::max_vowels("leetcode".to_string(), 3));
    }
}
