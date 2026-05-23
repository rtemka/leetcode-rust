// https://leetcode.com/problems/reverse-vowels-of-a-string/description
struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let (mut lo, mut hi) = (0, s.len() - 1);
        let mut sb = s.into_bytes();
        while lo < hi {
            let (lo_not_vowel, hi_not_vowel) = (
                !Self::is_vowel(sb[lo]) as usize,
                !Self::is_vowel(sb[hi]) as usize,
            );
            if lo_not_vowel + hi_not_vowel == 0 {
                sb.swap(lo, hi);
                (lo, hi) = (lo + 1, hi - 1);
            }
            (lo, hi) = (lo + lo_not_vowel, hi - hi_not_vowel);
        }
        unsafe { String::from_utf8_unchecked(sb) }
    }

    #[inline]
    fn is_vowel(c: u8) -> bool {
        match c {
            b'a' | b'e' | b'i' | b'o' | b'u' => true,
            b'A' | b'E' | b'I' | b'O' | b'U' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_vowels() {
        assert_eq!(
            "holle".to_string(),
            Solution::reverse_vowels("hello".to_string())
        );
        assert_eq!(
            "leotcede".to_string(),
            Solution::reverse_vowels("leetcode".to_string())
        );
    }
}
