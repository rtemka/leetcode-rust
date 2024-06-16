// https://leetcode.com/problems/generate-parentheses/description/
struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut v = Vec::new();
        Self::backtrack(n, 0, 0, &mut v, &mut String::with_capacity(n * 2));
        v
    }

    fn backtrack(n: usize, open: usize, close: usize, res: &mut Vec<String>, s: &mut String) {
        if s.len() == n * 2 {
            res.push(s.clone());
            return;
        }
        if open < n {
            s.push('(');
            Self::backtrack(n, open + 1, close, res, s);
            s.pop();
        }
        if close < open {
            s.push(')');
            Self::backtrack(n, open, close + 1, res, s);
            s.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_parenthesis() {
        assert_eq!(
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"],
            Solution::generate_parenthesis(3)
        );
        assert_eq!(vec!["(())", "()()"], Solution::generate_parenthesis(2));
    }
}
