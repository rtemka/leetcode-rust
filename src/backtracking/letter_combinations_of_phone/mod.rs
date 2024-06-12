// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
struct Solution;

const PHONE_KEYBOARD: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
const OFFSET: u8 = 0x32; // 50; so for example "2" as ASCII == 50 - 50 becomes 0 index in
                         // PHONE_KEYBOARD

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mut res = Vec::with_capacity(Self::count_total_vec_len(&digits));
        Self::backtrack(&digits, &mut res, &mut String::with_capacity(digits.len()));
        // println!("{:?}\tlen={}", res, res.len());
        res
    }

    #[inline]
    fn backtrack(digits: &str, res: &mut Vec<String>, s: &mut String) {
        if digits.is_empty() {
            res.push(s.clone());
            return;
        }
        let digit = (digits.chars().nth(0).unwrap() as u8 - OFFSET) as usize;
        for c in PHONE_KEYBOARD[digit].chars() {
            s.push(c);
            Self::backtrack(&digits[1..], res, s);
            s.pop();
        }
    }

    #[inline]
    fn count_total_vec_len(digits: &str) -> usize {
        digits.chars().fold(1, |acc, c| {
            acc * PHONE_KEYBOARD[(c as u8 - OFFSET) as usize].len()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_combinations() {
        assert_eq!(
            vec![
                "ad".to_string(),
                "ae".to_string(),
                "af".to_string(),
                "bd".to_string(),
                "be".to_string(),
                "bf".to_string(),
                "cd".to_string(),
                "ce".to_string(),
                "cf".to_string(),
            ],
            Solution::letter_combinations("23".to_string())
        );

        assert_eq!(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            Solution::letter_combinations("2".to_string())
        );

        assert!(Solution::letter_combinations("".to_string()).is_empty());

        assert_eq!(
            vec![
                "atw".to_string(),
                "atx".to_string(),
                "aty".to_string(),
                "atz".to_string(),
                "auw".to_string(),
                "aux".to_string(),
                "auy".to_string(),
                "auz".to_string(),
                "avw".to_string(),
                "avx".to_string(),
                "avy".to_string(),
                "avz".to_string(),
                "btw".to_string(),
                "btx".to_string(),
                "bty".to_string(),
                "btz".to_string(),
                "buw".to_string(),
                "bux".to_string(),
                "buy".to_string(),
                "buz".to_string(),
                "bvw".to_string(),
                "bvx".to_string(),
                "bvy".to_string(),
                "bvz".to_string(),
                "ctw".to_string(),
                "ctx".to_string(),
                "cty".to_string(),
                "ctz".to_string(),
                "cuw".to_string(),
                "cux".to_string(),
                "cuy".to_string(),
                "cuz".to_string(),
                "cvw".to_string(),
                "cvx".to_string(),
                "cvy".to_string(),
                "cvz".to_string(),
            ],
            Solution::letter_combinations("289".to_string())
        );
    }
}
