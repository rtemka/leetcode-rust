// #419
// https://leetcode.com/problems/battleships-in-a-board

#![allow(clippy::needless_range_loop)]

struct Solution;

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let (m, n) = (board.len(), board[0].len());

        for row in 0..m {
            for col in 0..n {
                if board[row][col] == 'X'
                    && (row == 0 || board[row - 1][col] != 'X')
                    && (col == 0 || board[row][col - 1] != 'X')
                {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn count_battleships_my_thoughts(board: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let (m, n) = (board.len(), board[0].len());

        // Count all horizontally placed ships plus singular ships
        for row in 0..m {
            let mut ships = 0;
            let mut length = 0;

            for col in 0..n {
                if board[row][col] == '.' && length > 0 {
                    ships += 1;
                    length = 0;
                }
                if board[row][col] == 'X'
                    && (row == 0 || board[row - 1][col] != 'X')
                    && (row + 1 == m || board[row + 1][col] != 'X')
                {
                    length += 1;
                }
            }

            count += ships + (length > 0) as i32;
        }

        // Count all vertically placed ships minus singular ships
        // because they were counted in a loop above
        for col in 0..n {
            let mut ships = 0;
            let mut length = 0;

            for row in 0..m {
                if board[row][col] == '.' && length > 0 {
                    ships += 1;
                    length = 0;
                }

                if board[row][col] == 'X'
                    && (col == 0 || board[row][col - 1] == '.')
                    && (col + 1 == n || board[row][col + 1] == '.')
                    && (row == 0 || board[row - 1][col] == '.')
                    && (row + 1 == m || board[row + 1][col] == '.')
                {
                    continue;
                }

                if board[row][col] == 'X'
                    && (col == 0 || board[row][col - 1] != 'X')
                    && (col + 1 == n || board[row][col + 1] != 'X')
                {
                    length += 1;
                }
            }

            count += ships + (length > 0) as i32;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn battleships_in_a_board() {
        assert_eq!(
            2,
            Solution::count_battleships(vec![
                vec!['X', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
                vec!['.', '.', '.', 'X']
            ])
        );

        assert_eq!(0, Solution::count_battleships(vec![vec!['.'],]));
    }
}
