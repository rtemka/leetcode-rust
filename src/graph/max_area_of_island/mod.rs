// #695
// https://leetcode.com/problems/max-area-of-island/description/

struct Solution;

const DIRS: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut max_area = 0;
        let mut stack = Vec::new();

        for row in 0..m {
            for col in 0..n {
                if grid[row][col] != 1 {
                    continue;
                }

                max_area = max_area.max(Self::dfs(&mut grid, (row, col), &mut stack))
            }
        }

        max_area
    }

    #[inline]
    fn dfs(grid: &mut [Vec<i32>], root: (usize, usize), stack: &mut Vec<(usize, usize)>) -> i32 {
        let mut count = 0;

        stack.push(root);

        while let Some(cell) = stack.pop() {
            if grid[cell.0][cell.1] != 1 {
                continue;
            }

            count += 1;
            grid[cell.0][cell.1] = -1;

            for dir in DIRS {
                let coords = (cell.0 as isize + dir.0, cell.1 as isize + dir.1);
                if let Some(neighbor) = Self::get_neighbor(grid, coords) {
                    stack.push(neighbor);
                }
            }
        }

        count
    }

    #[inline]
    fn get_neighbor(grid: &[Vec<i32>], coords: (isize, isize)) -> Option<(usize, usize)> {
        if coords.0 >= 0
            && (coords.0 as usize) < grid.len()
            && coords.1 >= 0
            && (coords.1 as usize) < grid[0].len()
        {
            Some((coords.0 as usize, coords.1 as usize))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_area_of_island() {
        assert_eq!(
            6,
            Solution::max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ])
        );
    }
}
