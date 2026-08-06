// #696
// https://leetcode.com/problems/count-binary-substrings/description/

struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut result = 0;
        let s = s.as_bytes();

        // Keep track of the `ones` and `zeroes`.
        let mut ones = 0;
        let mut zeroes = 0;

        for (i, &c) in s.iter().enumerate() {
            // If there is a switch from '0' -> '1' then drop the count of `ones`.
            // If there is a switch from '1' -> '0' then drop the count of `zeroes`.
            if i > 0 && s[i] != s[i - 1] {
                ones = if c == b'1' { 0 } else { ones };
                zeroes = if c == b'0' { 0 } else { zeroes };
            }

            // We will add to the result if there is a pair of "0"/"1" from which we can subtract.
            match c {
                b'1' => {
                    if zeroes > 0 {
                        result += 1;
                        zeroes -= 1;
                    }
                    ones += 1;
                }
                b'0' => {
                    if ones > 0 {
                        result += 1;
                        ones -= 1;
                    }
                    zeroes += 1;
                }
                _ => (),
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_binary_substrings() {
        assert_eq!(6, Solution::count_binary_substrings("00110011".to_string()));
        assert_eq!(4, Solution::count_binary_substrings("10101".to_string()));
        assert_eq!(3, Solution::count_binary_substrings("10011".to_string()));
        assert_eq!(3, Solution::count_binary_substrings("000111".to_string()));
        assert_eq!(2, Solution::count_binary_substrings("11100".to_string()));
        assert_eq!(5, Solution::count_binary_substrings("00011100".to_string()));
    }
}
