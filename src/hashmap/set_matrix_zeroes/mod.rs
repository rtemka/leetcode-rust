// #73
// https://leetcode.com/problems/set-matrix-zeroes/description
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        Self::set_zeroes_in_place(matrix);
    }

    #[inline]
    fn set_zeroes_hashmap(matrix: &mut Vec<Vec<i32>>) {
        let mut zero_rows: HashSet<usize> = HashSet::new();
        let mut zero_cols: HashSet<usize> = HashSet::new();
        // Pass 1: Traverse through the matrix to identify the rows and
        // columns containing zeros and store their indexes in the appropriate hash sets.
        for r in 0..matrix.len() {
            for c in 0..matrix[r].len() {
                if matrix[r][c] == 0 {
                    zero_rows.insert(r);
                    zero_cols.insert(c);
                }
            }
        }
        // Pass 2: Set any cell in the matrix to zero if its row index is
        // in 'zero_rows' or its column index is in 'zero_cols'.
        for r in 0..matrix.len() {
            for c in 0..matrix[r].len() {
                if zero_rows.contains(&r) || zero_cols.contains(&c) {
                    matrix[r][c] = 0;
                }
            }
        }
    }

    #[inline]
    fn set_zeroes_in_place(matrix: &mut Vec<Vec<i32>>) {
        // Check if the first row contains a zero.
        let first_row_has_zero = matrix[0].iter().any(|&v| v == 0);
        // Check if the first column contains a zero.
        let first_col_has_zero = matrix.iter().any(|v| v[0] == 0);
        // We use first row and first column to keep the state of the corresponding row and column.
        // If an element in the matrix[n-1*m-1] is zero, mark its corresponding row and column
        // in the first row and column as 0.
        for r in 1..matrix.len() {
            for c in 1..matrix[r].len() {
                if matrix[r][c] == 0 {
                    matrix[0][c] = 0;
                    matrix[r][0] = 0;
                }
            }
        }
        // Set the matrix cell to zero if first row or column cell contains zero.
        for r in 1..matrix.len() {
            for c in 1..matrix[r].len() {
                if matrix[0][c] == 0 || matrix[r][0] == 0 {
                    matrix[r][c] = 0;
                }
            }
        }
        // If the first row had a zero initially, set all elements in the first row to zero.
        if first_row_has_zero {
            matrix[0].iter_mut().for_each(|v| *v = 0);
        }
        // If the first column had a zero initially, set all elements in the first column to zero.
        if first_col_has_zero {
            matrix.iter_mut().for_each(|v| v[0] = 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_matrix_zeroes() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]], matrix)
    }
}
