// #583
// https://leetcode.com/problems/delete-operation-for-two-strings/description/
// See: #1143 Longest Common Subsequence
// See: #72 Edit Disctance
struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // Self::levenstein(word1, word2) as i32
        (word1.len() + word2.len()) as i32 - Self::lcs(word1, word2) * 2
    }

    fn levenstein(word1: String, word2: String) -> usize {
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (word1.len(), word2.len());

        let mut matrix = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            matrix[i][0] = i;
        }
        for j in 1..=n {
            matrix[0][j] = j;
        }
        // For each "combination" of word1, word2.
        for i in 1..=m {
            for j in 1..=n {
                // If the characters are equal then its just the sum of deleting the previous characters.
                if word1[i - 1] == word2[j - 1] {
                    matrix[i][j] = matrix[i - 1][j - 1];
                } else {
                    // Else its the min of either deleting the character from word1 or word2.
                    matrix[i][j] = 1 + usize::min(matrix[i - 1][j], matrix[i][j - 1]);
                }
            }
        }

        matrix[m][n]
    }

    fn lcs(word1: String, word2: String) -> i32 {
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (word1.len(), word2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                if word1[i - 1] == word2[j - 1] {
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
    fn delete_operation_for_two_strings() {
        assert_eq!(
            4,
            Solution::min_distance("leetcode".to_string(), "etco".to_string())
        );
        assert_eq!(
            8,
            Solution::min_distance("artem".to_string(), "zzzzm".to_string())
        );
        assert_eq!(
            10,
            Solution::min_distance("abcde".to_string(), "ihfjk".to_string())
        );
        assert_eq!(2, Solution::min_distance("a".to_string(), "t".to_string()));
        assert_eq!(
            2,
            Solution::min_distance("sea".to_string(), "eat".to_string())
        );
    }
}
