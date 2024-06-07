struct Solution;

const ASCII_A_LOWER: usize = 97;
const ASCII_Z_LOWER: usize = 122;

// https://leetcode.com/problems/first-unique-character-in-a-string/description
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut lower_ascii = [0; ASCII_Z_LOWER - ASCII_A_LOWER + 1];
        for c in s.chars() {
            lower_ascii[c as usize - ASCII_A_LOWER] += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if lower_ascii[c as usize - ASCII_A_LOWER] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_unique_char() {
        assert_eq!(0, Solution::first_uniq_char("leetcode".to_string()));
        assert_eq!(2, Solution::first_uniq_char("loveleetcode".to_string()));
        assert_eq!(-1, Solution::first_uniq_char("aabb".to_string()));
        assert_eq!(4, Solution::first_uniq_char("aabbz".to_string()));
    }
}
