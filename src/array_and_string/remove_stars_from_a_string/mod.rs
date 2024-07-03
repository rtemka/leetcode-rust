// https://leetcode.com/problems/removing-stars-from-a-string
struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut v: Vec<char> =
            Vec::with_capacity(s.len() - s.chars().fold(0, |acc, x| acc + (x == '*') as usize) * 2);
        for c in s.chars() {
            if c == '*' {
                v.pop();
            } else {
                v.push(c);
            }
        }
        v.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_stars() {
        assert_eq!(
            "lecoe".to_string(),
            Solution::remove_stars("leet**cod*e".to_string())
        );
        assert_eq!(
            "".to_string(),
            Solution::remove_stars("erase*****".to_string())
        );
        assert_eq!(
            "".to_string(),
            Solution::remove_stars("a*b*c*d*e*".to_string())
        );
        assert_eq!("e".to_string(), Solution::remove_stars("*e".to_string()));
    }
}
