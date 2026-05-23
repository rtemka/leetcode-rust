// https://leetcode.com/problems/rotate-image/description/
struct Solution;

impl Solution {
    #[allow(clippy::all)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        //     Constraints:
        //
        // n == matrix.length == matrix[i].length
        // 1 <= n <= 20
        // -1000 <= matrix[i][j] <= 1000
        //
        let n = matrix[0].len() - 1;

        // let sep = "-".repeat(3 + ((n) * 2) + n);
        // println!("{sep}");
        // for row in matrix.iter() {
        //     println!("{:?}", row);
        // }
        // println!("{sep}");

        // amount of rotation steps needed = matrix.len / 2
        for k in 0..((n + 1) / 2) {
            for i in k..n - k {
                (matrix[k][i], matrix[i][n - k]) = (matrix[i][n - k], matrix[k][i]);

                (matrix[k][i], matrix[n - k][n - i]) = (matrix[n - k][n - i], matrix[k][i]);

                (matrix[k][i], matrix[n - i][k]) = (matrix[n - i][k], matrix[k][i]);
            }
        }

        // println!("{sep}");
        // for row in matrix.iter() {
        //     println!("{:?}", row);
        // }
        // println!("{sep}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_image() {
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(expected, matrix);

        let expected = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(expected, matrix);
    }
}
