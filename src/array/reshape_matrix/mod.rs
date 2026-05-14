// #566
// https://leetcode.com/problems/reshape-the-matrix/description/

struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let (r, c) = (r as usize, c as usize);

        if m * n != r * c || (m == r && n == c) {
            return mat;
        }

        let (mut dst_row, mut dst_col) = (0, 0);
        let mut res = vec![vec![0_i32; c]; r];

        for src_row in 0..m {
            for src_col in 0..n {
                res[dst_row][dst_col] = mat[src_row][src_col];

                dst_col = dst_col + 1;
                if dst_col == c {
                    dst_col = 0;
                    dst_row = dst_row + 1;
                };
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reshape_matrix() {
        assert_eq!(
            vec![vec![1, 2, 3, 4]],
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4)
        );

        assert_eq!(
            vec![vec![1, 2], vec![3, 4]],
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4)
        );

        assert_eq!(
            vec![vec![1, 2, 3], vec![4, 5, 6]],
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 3)
        );
    }
}
