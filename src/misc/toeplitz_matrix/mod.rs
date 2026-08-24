// #766
// https://leetcode.com/problems/toeplitz-matrix/description/

struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for row in 0..matrix.len() {
            for col in 0..matrix[row].len() {
                if row > 0 && col > 0 && matrix[row][col] != matrix[row - 1][col - 1] {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toeplitz_matrix() {
        assert!(Solution::is_toeplitz_matrix(vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![9, 5, 1, 2]
        ]));
    }
}
