// #72
// https://leetcode.com/problems/edit-distance/description
struct Solution;

// https://en.wikipedia.org/wiki/Levenshtein_distance

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (word1.len(), word2.len());

        // Create a 2D array to store the dynamic programming results.
        let mut matrix = vec![vec![0; n + 1]; m + 1];
        // Initialize the base cases.
        for i in 1..=m {
            matrix[i][0] = i;
        }
        for j in 1..=n {
            matrix[0][j] = j;
        }

        for i in 1..=m {
            for j in 1..=n {
                let replace_cost = if word1[i - 1] == word2[j - 1] { 0 } else { 1 };
                matrix[i][j] = usize::min(
                    matrix[i][j - 1] + 1, // insert
                    usize::min(
                        matrix[i - 1][j] + 1,                // delete
                        matrix[i - 1][j - 1] + replace_cost, // replace
                    ),
                )
            }
        }

        matrix[m][n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edit_distance() {
        assert_eq!(
            3,
            Solution::min_distance("horse".to_string(), "ros".to_string())
        );
        assert_eq!(
            5,
            Solution::min_distance("intention".to_string(), "execution".to_string())
        );
    }
}
