// https://leetcode.com/problems/greatest-common-divisor-of-strings/description
struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let gcd = Self::gcd(str1.len(), str2.len());
        println!("gcd={}\tstr1={}\tstr2={}", gcd, &str1[..gcd], &str2[..gcd]);
        let gcd_str = &str1[..gcd];
        if str1[..gcd] != str2[..gcd]
            || !Self::contains_all(gcd_str, &str2)
            || !Self::contains_all(gcd_str, &str1)
        {
            return "".to_string();
        }
        gcd_str.to_string()
    }

    #[inline]
    fn gcd(mut x: usize, mut y: usize) -> usize {
        while y != 0 {
            (x, y) = (y, x % y);
        }
        x
    }

    #[inline]
    fn contains_all(prefix: &str, s: &str) -> bool {
        let mut i = 0;
        while i < s.len() {
            println!(
                "i={}\ts={}\tprefix={}\tres={}",
                i,
                &s[i..i + prefix.len()],
                prefix,
                s[i..i + prefix.len()] != *prefix
            );
            if s[i..i + prefix.len()] != *prefix {
                return false;
            }
            i += prefix.len();
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_of_strings() {
        assert_eq!(
            "".to_string(),
            Solution::gcd_of_strings("A".to_string(), "B".to_string())
        );
        assert_eq!(
            "A".to_string(),
            Solution::gcd_of_strings("A".to_string(), "A".to_string())
        );
        assert_eq!(
            "".to_string(),
            Solution::gcd_of_strings("ABCDEF".to_string(), "ABCXYJ".to_string())
        );
        assert_eq!(
            "".to_string(),
            Solution::gcd_of_strings("ABCDEF".to_string(), "ABC".to_string())
        );
        assert_eq!(
            "".to_string(),
            Solution::gcd_of_strings("ABABABAB".to_string(), "ABA".to_string())
        );
        assert_eq!(
            "ABC".to_string(),
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string())
        );
        assert_eq!(
            "AB".to_string(),
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string())
        );
        assert_eq!(
            "".to_string(),
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string())
        );
        assert_eq!(
            "TAUXX".to_string(),
            Solution::gcd_of_strings(
                "TAUXXTAUXXTAUXXTAUXXTAUXX".to_string(),
                "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX".to_string()
            )
        );
    }
}
