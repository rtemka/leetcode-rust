// #64
// https://leetcode.com/problems/minimum-path-sum/description
struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for i in 0..m {
            for j in 0..n {
                let cost = match (i, j) {
                    (0, 0) => 0,
                    (0, _) => grid[i][j - 1],
                    (_, 0) => grid[i - 1][j],
                    _ => i32::min(grid[i - 1][j], grid[i][j - 1]),
                };
                grid[i][j] = grid[i][j] + cost;
            }
        }

        grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_path_sum() {
        assert_eq!(
            7,
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        );
    }
}
