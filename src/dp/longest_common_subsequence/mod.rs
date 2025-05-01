// #1143
// https://leetcode.com/problems/longest-common-subsequence/description/
struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        println!("{:?}, {:?}", text1, text2);
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
        let (m, n) = (text1.len(), text2.len());
        let mut dp = vec![vec![0i32; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                // // My initial solution
                // let carry = if text1[i - 1] == text2[j - 1] { 1 } else { 0 };
                // dp[i][j] = carry + i32::max(dp[i - 1][j - 1], i32::max(dp[i - 1][j], dp[i][j - 1]));

                if text1[i - 1] == text2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = i32::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }

        super::print_table(&dp);

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_common_subsequence() {
        assert_eq!(
            3,
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string())
        );
        assert_eq!(
            3,
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string())
        );
        assert_eq!(
            0,
            Solution::longest_common_subsequence("abc".to_string(), "def".to_string())
        );
    }
}
