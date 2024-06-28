// https://leetcode.com/problems/palindrome-partitioning/description/
struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = Vec::new();
        Self::backtrack(&s, &mut res, &mut Vec::new());
        res
    }

    fn backtrack<'a>(s: &'a str, res: &mut Vec<Vec<String>>, cur: &mut Vec<String>) {
        println!("s={}\tres={:?}\tcur={:?}", s, res, cur);
        if s.is_empty() {
            res.push(cur.clone());
        } else {
            for i in 0..s.len() {
                let substr = &s[0..=i];
                if !Self::is_palindrome(substr) {
                    continue;
                }
                cur.push(substr.to_string());
                Self::backtrack(&s[i + 1..], res, cur);
                cur.pop();
            }
        }
    }

    fn is_palindrome(s: &str) -> bool {
        let mut i = s.chars();
        loop {
            match (i.next(), i.next_back()) {
                (Some(a), Some(b)) if a == b => continue,
                (_, None) | (None, _) => return true,
                (_, _) => return false,
            }
        }
    }

    fn is_palindrome2(s: &str) -> bool {
        if s.len() < 2 {
            return true;
        }
        s[0..s.len() / 2]
            .chars()
            .eq(s[s.len() / 2..s.len()].chars().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome() {
        assert!(Solution::is_palindrome2("abba"));
        assert!(Solution::is_palindrome("aa"));
        assert!(Solution::is_palindrome("a"));
        assert!(Solution::is_palindrome("aaaggaaa"));
        assert!(Solution::is_palindrome2("aaaggaaa"));
    }

    #[test]
    fn palindrome_partition() {
        assert_eq!(
            vec![vec!["a".to_string()]],
            Solution::partition("a".to_string())
        );
        assert_eq!(
            vec![
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
                vec!["aa".to_string(), "b".to_string()],
            ],
            Solution::partition("aab".to_string()),
        );
        assert_eq!(
            vec![
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "b".to_string(),
                    "a".to_string(),
                    "b".to_string()
                ],
                vec!["a".to_string(), "b".to_string(), "bab".to_string()],
                vec![
                    "a".to_string(),
                    "bb".to_string(),
                    "a".to_string(),
                    "b".to_string()
                ],
                vec!["abba".to_string(), "b".to_string()]
            ],
            Solution::partition("abbab".to_string()),
        );
        assert_eq!(
            vec![
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string(),
                    "a".to_string()
                ],
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "aa".to_string()
                ],
            ],
            Solution::partition("abcaa".to_string()),
        );
    }
}
