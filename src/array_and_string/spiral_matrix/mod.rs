// https://leetcode.com/problems/spiral-matrix/description/
struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut up, mut down) = (0, matrix.len() - 1);
        let (mut left, mut right) = (0, matrix[0].len() - 1);
        let (mut m, mut n) = (0, 0);
        // corner cases!
        if up == down {
            return matrix[0].clone();
        }
        let mut res = Vec::with_capacity(matrix.len() * matrix[0].len());
        // corner cases!
        if left == right {
            while m < matrix.len() {
                res.push(matrix[m][n]);
                m += 1;
            }
            return res;
        }
        while res.len() != res.capacity() - 1 {
            match (m, n) {
                (_, _) if m == up && n == left => {
                    while n != right {
                        res.push(matrix[m][n]);
                        n += 1;
                    }
                    if up != 0 {
                        left += 1;
                    }
                    println!(
                        "{:?};up:{};right{};down:{};left{}",
                        res, up, right, down, left
                    );
                }
                (_, _) if m == up && n == right => {
                    while m != down {
                        res.push(matrix[m][n]);
                        m += 1;
                    }
                    up += 1;
                    println!(
                        "{:?};up:{};right{};down:{};left{}",
                        res, up, right, down, left
                    );
                }
                (_, _) if m == down && n == right => {
                    while n != left {
                        res.push(matrix[m][n]);
                        n -= 1;
                    }
                    right -= 1;
                    println!(
                        "{:?};up:{};right{};down:{};left{}",
                        res, up, right, down, left
                    );
                }
                (_, _) if m == down && n == left => {
                    while m != up {
                        res.push(matrix[m][n]);
                        m -= 1;
                    }
                    down -= 1;
                    println!(
                        "{:?};up:{};right{};down:{};left{}",
                        res, up, right, down, left
                    );
                }
                (_, _) => panic!("d'oh"),
            };
        }
        res.push(matrix[m][n]); // last number always leftover and we are at it;
        res
    }
}

//  1,  2,  3,  4
//  5,  6,  7,  8
//  9, 10, 11, 12
// 13, 14, 15, 16
// 17, 18, 19, 20

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spiral_order() {
        assert_eq!(vec![1], Solution::spiral_order(vec![vec![1]]));
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::spiral_order(vec![vec![1, 2, 3, 4]])
        );
        assert_eq!(
            vec![1, 2, 3],
            Solution::spiral_order(vec![vec![1], vec![2], vec![3]])
        );
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
        assert_eq!(
            vec![1, 2, 4, 3],
            Solution::spiral_order(vec![vec![1, 2], vec![3, 4]])
        );
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
            ])
        );
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 16, 20, 19, 18, 17, 13, 9, 5, 6, 7, 11, 15, 14, 10],
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
                vec![17, 18, 19, 20]
            ])
        );
    }
}
