// https://leetcode.com/problems/word-search/description/
struct Solution;

#[derive(Debug, Copy, Clone)]
struct Cell {
    m: usize,
    n: usize,
    // на самом деле можно не "запоминать" кого мы посещали,
    // а просто заменять пройденные буквы на #
    // (если будем возвращаться, то меняем обратно)
    // Это работает с рекурсивным подходом DFS
    l: bool,
    r: bool,
    u: bool,
    d: bool,
}

impl Cell {
    #[inline]
    fn new(m: usize, n: usize) -> Self {
        Self {
            m,
            n,
            l: true,
            r: true,
            u: true,
            d: true,
        }
    }
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        // println!("\n\n");
        if word.len() > board.len() * board[0].len() {
            return false;
        }
        let b_word = word.as_bytes();

        let mut stack: Vec<Cell> = Vec::with_capacity(b_word.len());

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] as u8 != b_word[0] {
                    continue;
                }

                let mut cell = Cell::new(i, j);
                let mut c = 1;

                while c < b_word.len() {
                    if !stack.iter().any(|e| e.m == cell.m && e.n == cell.n) {
                        stack.push(cell);
                    }
                    let last_idx = stack.len() - 1;
                    // println!(
                    //     "stack:{};the_c={};c={};cell.m={};cell.n={}",
                    //     stack.len(),
                    //     b_word[c] as char,
                    //     c,
                    //     cell.m,
                    //     cell.n
                    // );
                    (cell, c) = if cell.m > 0 // goes UP!
                        && cell.u
                        && board[cell.m - 1][cell.n] as u8 == b_word[c]
                        && !stack.iter().any(|e| e.m == cell.m-1 && e.n == cell.n)
                    {
                        // println!("GOES UP!");
                        cell = Cell::new(cell.m - 1, cell.n);
                        cell.d = false;
                        stack[last_idx].u = false;
                        (cell, c + 1)
                    } else if cell.n < board[i].len() - 1 // goes RIGHT!
                        && cell.r
                        && board[cell.m][cell.n + 1] as u8 == b_word[c]
                        && !stack.iter().any(|e| e.m == cell.m && e.n == cell.n+1)
                    {
                        // println!("GOES RIGHT!");
                        cell = Cell::new(cell.m, cell.n + 1);
                        cell.l = false;
                        stack[last_idx].r = false;
                        (cell, c + 1)
                    } else if cell.m < board.len() - 1 // goes DOWN!
                        && cell.d
                        && board[cell.m +1][cell.n] as u8 == b_word[c]
                        && !stack.iter().any(|e| e.m == cell.m +1 && e.n == cell.n)
                    {
                        // println!("GOES DOWN!");
                        cell = Cell::new(cell.m + 1, cell.n);
                        cell.u = false;
                        stack[last_idx].d = false;
                        (cell, c + 1)
                    } else if cell.n > 0 // goes LEFT!
                        && cell.l
                        && board[cell.m][cell.n-1] as u8 == b_word[c]
                        && !stack.iter().any(|e| e.m == cell.m && e.n == cell.n-1)
                    {
                        // println!("GOES LEFT!");
                        cell = Cell::new(cell.m, cell.n - 1);
                        cell.r = false;
                        stack[last_idx].l = false;
                        (cell, c + 1)
                    } else {
                        // goes BACK!
                        // println!("GOES BACK!");
                        stack.pop();
                        if stack.is_empty() {
                            break;
                        }
                        cell = stack[stack.len() - 1];
                        (cell, c - 1)
                    };
                }
                if c == b_word.len() {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_word_search() {
        assert!(!Solution::exist(vec![vec!['A'],], "S".to_string(),));
        assert!(Solution::exist(vec![vec!['A'],], "A".to_string(),));
        assert!(Solution::exist(
            vec![vec!['A'], vec!['S'], vec!['A'],],
            "ASA".to_string(),
        ));
        assert!(!Solution::exist(
            vec![vec!['A', 'B', 'C', 'E'],],
            "CBAC".to_string(),
        ));
        assert!(!Solution::exist(
            vec![vec!['A', 'B'], vec!['A', 'B'],],
            "AABBAA".to_string(),
        ));
        assert!(!Solution::exist(
            vec![vec!['A', 'B', 'A']],
            "ABAB".to_string(),
        ));
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "SEE".to_string(),
        ));
        assert!(!Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCB".to_string(),
        ));
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCCED".to_string(),
        ));
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'E', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCEFSADEESE".to_string(),
        ));
    }
}
