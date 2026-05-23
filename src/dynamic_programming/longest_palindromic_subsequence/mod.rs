// #516
// https://leetcode.com/problems/longest-palindromic-subsequence/description/
// See also: #1143
struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let (m, n) = (s.len(), s.len());
        let mut dp = vec![vec![0i32; n + 1]; m + 1];

        // So, the solution is to apply LCS on the <s> and the reverse of the <s>.

        for i in 1..=m {
            for j in 1..=n {
                // Here we just take chars of <s> in reverse.
                let k = n - j + 1;
                if s[i - 1] == s[k - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = i32::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_palindromic_subsequence() {
        assert_eq!(4, Solution::longest_palindrome_subseq("bbbab".to_string()));
    }
}
