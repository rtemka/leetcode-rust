// #541
// https://leetcode.com/problems/01-matrix/description
struct Solution;

use std::collections::VecDeque;

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Queue for the BFS.
        let mut queue = VecDeque::new();
        // Memo for the visited cells.
        let mut visited = vec![vec![-1; mat[0].len()]; mat.len()];

        // Gather all '0' cells from the matrix.
        for row in 0..mat.len() {
            for col in 0..mat[row].len() {
                if mat[row][col] == 0 {
                    queue.push_back((row, col));
                    visited[row][col] = 0;
                }
            }
        }
        // Perfom multi-source BFS from '0' cells continuously updating
        // adjacent '1' cells with the shortest distance to '0'.
        let mut distance = 0;
        while !queue.is_empty() {
            // Add 1 to the distance with each level of the matrix that's explored.
            distance += 1;
            for _ in 0..queue.len() {
                if let Some((r, c)) = queue.pop_front() {
                    // Update any neighboring cells and add them to the queue to be processed in
                    // the next level.
                    for &dir in &DIRS {
                        if let Some((next_r, next_c)) = Self::is_within_bounds(r, c, dir, &mat) {
                            if visited[next_r][next_c] < 0 && mat[next_r][next_c] == 1 {
                                mat[next_r][next_c] = distance;
                                visited[next_r][next_c] = 0;
                                queue.push_back((next_r, next_c));
                            }
                        }
                    }
                }
            }
        }
        mat
    }

    #[inline]
    fn is_within_bounds(
        row: usize,
        col: usize,
        dir: (isize, isize),
        matrix: &Vec<Vec<i32>>,
    ) -> Option<(usize, usize)> {
        let row = (row as isize).checked_add(dir.0)? as usize;
        let col = (col as isize).checked_add(dir.1)? as usize;
        if row < matrix.len() && col < matrix[0].len() {
            Some((row, col))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_one_matrix() {
        assert_eq!(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]])
        );

        assert_eq!(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]],
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]])
        );
    }
}
