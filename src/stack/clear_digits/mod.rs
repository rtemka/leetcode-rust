// https://leetcode.com/problems/clear-digits/description
struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut res = String::new();
        for c in s.chars() {
            if ('0'..='9').contains(&c) {
                res.pop();
            } else {
                res.push(c);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_digits() {
        assert_eq!(
            "".to_string(),
            Solution::clear_digits("acb34de568".to_string())
        );
        assert_eq!(
            "".to_string(),
            Solution::clear_digits("cb34545456456232".to_string())
        );
        assert_eq!(
            "xjkpaaovhqckjhrndtbobgeke".to_string(),
            Solution::clear_digits("qm93xjkpaaovhqckjhg5j1o4rndtg3bobgeke".to_string())
        );
        assert_eq!("a".to_string(), Solution::clear_digits("a".to_string()));
        assert_eq!(
            "xzuzghilydk".to_string(),
            Solution::clear_digits("xzuzr2ghilydk".to_string())
        );
        assert_eq!("".to_string(), Solution::clear_digits("cb34".to_string()));
        assert_eq!("abc".to_string(), Solution::clear_digits("abc".to_string()));
    }
}
