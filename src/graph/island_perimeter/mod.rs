// #463
// https://leetcode.com/problems/island-perimeter/description/

struct Solution;

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut result = 0;

        for row in 0..m {
            for col in 0..n {
                if grid[row][col] != 1 {
                    continue;
                }

                for dir in DIRS {
                    let (r, c) = (row as isize + dir.0, col as isize + dir.1);
                    result += Self::get(&grid, r, c);
                }
            }
        }

        result
    }

    #[inline]
    fn get(grid: &[Vec<i32>], row: isize, col: isize) -> i32 {
        if row >= 0
            && (row as usize) < grid.len()
            && col >= 0
            && (col as usize) < grid[0].len()
            && grid[row as usize][col as usize] == 1
        {
            0
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn island_perimeter() {
        assert_eq!(
            16,
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ])
        );
    }
}
