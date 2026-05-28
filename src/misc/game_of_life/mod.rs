// #289
// https://leetcode.com/problems/game-of-life/description/

struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut [Vec<i32>]) {
        let (n, m) = (board.len(), board[0].len());
        let mut wider_board = vec![vec![0; m + 2]; n + 2];

        for row in 0..n {
            // This is the same as below
            wider_board[row + 1][1..m + 1].copy_from_slice(&board[row]);

            // for col in 0..board[row].len() {
            //     wider_board[row + 1][col + 1] = board[row][col];
            // }
        }

        // println!("{:?}", wider_board);

        for row in 0..board.len() {
            for col in 0..board[row].len() {
                let alive_neighbors = wider_board[row][col]
                    + wider_board[row + 1][col]
                    + wider_board[row + 2][col]
                    + wider_board[row][col + 1]
                    + wider_board[row][col + 2]
                    + wider_board[row + 1][col + 2]
                    + wider_board[row + 2][col + 1]
                    + wider_board[row + 2][col + 2];

                // println!("alive={}; cell_state={}", alive_neighbors, board[row][col]);

                board[row][col] = match (board[row][col], alive_neighbors) {
                    (1, ..2) | (1, 4..) => 0,
                    (1, 2..=3) => 1,
                    (0, 3) => 1,
                    (0, _) => 0,
                    _ => unreachable!(),
                };
            }
        }
    }

    //https://www.youtube.com/watch?v=fei4bJQdBUQ
    pub fn game_of_life_optimized(board: &mut [Vec<i32>]) {
        let (rows, columns) = (board.len(), board[0].len());

        // # Table of temporary states
        // Original | New | State
        //      0   |  0  |  0
        //      1   |  0  |  1
        //      0   |  1  |  2
        //      1   |  1  |  3

        // Mark cells with new states
        for row in 0..rows {
            for col in 0..columns {
                let alive = Self::alive_neighbors(board, row, col);

                board[row][col] = match (board[row][col], alive) {
                    (1, 2 | 3) => 3,
                    (0, 3) => 2,
                    (0, _) => 0,
                    (1, _) => 1,
                    (_, _) => unreachable!(),
                };
            }
        }

        // Update to the final state
        for row in board.iter_mut().take(rows) {
            for val in row.iter_mut().take(columns) {
                *val = match *val {
                    1 => 0,
                    2 | 3 => 1,
                    n => n,
                };
            }
        }
    }

    #[inline]
    fn alive_neighbors(board: &[Vec<i32>], row: usize, col: usize) -> usize {
        let (rows, columns) = (board.len(), board[0].len());
        let mut result = 0;

        #[allow(clippy::needless_range_loop)]
        for r in row.saturating_sub(1)..row + 2 {
            for c in col.saturating_sub(1)..col + 2 {
                if (r == row && c == col) || r == rows || c == columns {
                    continue;
                }
                if [1, 3].contains(&board[r][c]) {
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_of_life() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]];
        let expected = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1]];

        Solution::game_of_life(&mut board);
        assert_eq!(expected, board);

        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]];
        let expected = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1]];

        Solution::game_of_life_optimized(&mut board);
        assert_eq!(expected, board);
    }
}
