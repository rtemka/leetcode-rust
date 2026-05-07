// #917
// https://leetcode.com/problems/reverse-only-letters/description/

struct Solution;

impl Solution {
    pub fn reverse_only_letters(mut s: String) -> String {
        let vec = unsafe { s.as_mut_vec() };

        let (mut lo, mut hi) = (0, vec.len() - 1);

        while hi > lo {
            println!("{lo}:{hi}");
            match (
                Self::is_english_letter(vec[lo]),
                Self::is_english_letter(vec[hi]),
            ) {
                (true, true) => {
                    vec.swap(lo, hi);
                    lo = lo + 1;
                    hi = hi - 1;
                }
                (false, false) => {
                    lo = lo + 1;
                    hi = hi - 1;
                }
                (false, true) => lo = lo + 1,
                (true, false) => hi = hi - 1,
            }
        }

        s
    }

    fn is_english_letter(char: u8) -> bool {
        (char >= 65 && char < 91) || (char >= 97 && char < 123)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reverse_only_letters() {
        assert_eq!(
            Solution::reverse_only_letters("#@1GWE".to_string()),
            "#@1EWG".to_string()
        );

        assert_eq!(
            Solution::reverse_only_letters("ab-cd".to_string()),
            "dc-ba".to_string()
        );

        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba".to_string()
        );

        assert_eq!(
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1ct-eeLg=ntse-T!".to_string()
        );
    }
}
