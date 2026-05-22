// #942
// https://leetcode.com/problems/di-string-match/description/

struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let (mut lo, mut hi) = (0_i32, s.len() as i32);
        let mut result = Vec::with_capacity(s.len() + 1);

        for val in s.chars() {
            match val {
                'I' => {
                    result.push(lo);
                    lo += 1;
                }
                'D' => {
                    result.push(hi);
                    hi -= 1;
                }
                _ => unreachable!(),
            }
        }

        result.push(hi);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn di_stirng_match() {
        assert_eq!(
            vec![0, 4, 1, 3, 2],
            Solution::di_string_match("IDID".to_string())
        );

        assert_eq!(
            vec![0, 1, 2, 3],
            Solution::di_string_match("III".to_string())
        );

        assert_eq!(
            vec![3, 2, 1, 0],
            Solution::di_string_match("DDD".to_string())
        );

        assert_eq!(vec![0, 1], Solution::di_string_match("I".to_string()));
        assert_eq!(vec![1, 0], Solution::di_string_match("D".to_string()));

        assert_eq!(
            vec![3, 2, 0, 1],
            Solution::di_string_match("DDI".to_string())
        );
    }
}
