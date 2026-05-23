// https://leetcode.com/problems/merge-strings-alternately/description
struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let (mut lw, mut rw) = (word1.chars(), word2.chars());
        let mut s = String::with_capacity(word1.len() + word2.len());
        loop {
            match (lw.next(), rw.next()) {
                (Some(c1), Some(c2)) => {
                    s.push(c1);
                    s.push(c2)
                }
                (None, Some(c2)) => s.push(c2),
                (Some(c1), None) => s.push(c1),
                (None, None) => break,
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_alternately() {
        assert_eq!(
            "apbqcr".to_string(),
            Solution::merge_alternately("abc".to_string(), "pqr".to_string())
        );
        assert_eq!(
            "apbqrs".to_string(),
            Solution::merge_alternately("ab".to_string(), "pqrs".to_string())
        );
        assert_eq!(
            "apbqcd".to_string(),
            Solution::merge_alternately("abcd".to_string(), "pq".to_string())
        );
    }
}
