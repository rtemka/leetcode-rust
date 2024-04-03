// https://leetcode.com/problems/diagonal-traverse/description/
struct Solution {}

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut up = true;
        let (mut i, mut j) = (0, 0);
        let mut res = Vec::with_capacity(mat[0].len() * mat.len());
        let i_len = mat.len() - 1;
        let j_len = mat[0].len() - 1;
        while res.len() != res.capacity() {
            res.push(mat[i][j]);
            if up {
                while i > 0 && j < j_len {
                    (i, j) = (i - 1, j + 1);
                    res.push(mat[i][j]);
                }
            } else {
                while j > 0 && i < i_len {
                    (i, j) = (i + 1, j - 1);
                    res.push(mat[i][j]);
                }
            }
            (i, j) = match (i, j) {
                (0, _) if j == j_len => (i + 1, j),
                (0, _) => (i, j + 1),
                (_, _) if i == i_len => (i, j + 1),
                (_, _) => (i + 1, j),
            };
            up = !up;
            // println!("{:?}", res);
        }
        res
    }
}

//  1,  2,  3,  4
//  5,  6,  7,  8
//  9, 10, 11, 12
// 13, 14, 15, 16

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_diagonal_order() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::find_diagonal_order(vec![vec![1, 2, 3, 4]])
        );
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::find_diagonal_order(vec![vec![1], vec![2], vec![3], vec![4]])
        );
        assert_eq!(vec![1], Solution::find_diagonal_order(vec![vec![1]]));
        assert_eq!(
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9],
            Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::find_diagonal_order(vec![vec![1, 2], vec![3, 4]])
        );
        assert_eq!(
            vec![1, 2, 5, 9, 6, 3, 4, 7, 10, 13, 14, 11, 8, 12, 15, 16],
            Solution::find_diagonal_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16]
            ])
        );
    }
}
