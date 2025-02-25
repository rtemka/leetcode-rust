// https://leetcode.com/problems/unique-paths/description
struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut memo = vec![vec![-1i32; n]; m];
        Self::unique_paths_rec(m, n, (0, 0), &mut memo)
    }

    pub fn unique_paths_rec(
        m: usize,
        n: usize,
        cur: (usize, usize),
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if memo[cur.0][cur.1] >= 0 {
            return memo[cur.0][cur.1];
        }
        if cur == (m - 1, n - 1) {
            return 1;
        }
        let down = if cur.0 + 1 < m {
            Self::unique_paths_rec(m, n, (cur.0 + 1, cur.1), memo)
        } else {
            0
        };
        let right = if cur.1 + 1 < n {
            Self::unique_paths_rec(m, n, (cur.0, cur.1 + 1), memo)
        } else {
            0
        };
        memo[cur.0][cur.1] = down + right;
        down + right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_paths() {
        assert_eq!(1, Solution::unique_paths(1, 2));
        assert_eq!(28, Solution::unique_paths(3, 7));
        assert_eq!(3, Solution::unique_paths(3, 2));
        assert_eq!(1, Solution::unique_paths(1, 1));
    }
}
