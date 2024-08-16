use std::collections::{HashMap, VecDeque};

// https://leetcode.com/problems/nearest-exit-from-entrance-in-maze/description
struct Solution;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let entrance = (entrance[0] as usize, entrance[1] as usize);
        let mut trace: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut q = VecDeque::new();

        trace.insert(entrance, entrance);
        q.push_back(entrance);

        while let Some(cell) = q.pop_front() {
            if cell != entrance
                && (cell.0 == 0
                    || cell.0 + 1 == maze.len()
                    || cell.1 == 0
                    || cell.1 + 1 == maze[0].len())
            {
                return Self::steps_to_entrance(&trace, entrance, cell);
            }

            Self::explore_cell(
                &mut q,
                &mut trace,
                &maze,
                cell,
                (cell.0.saturating_sub(1), cell.1),
            );
            Self::explore_cell(&mut q, &mut trace, &maze, cell, (cell.0 + 1, cell.1));
            Self::explore_cell(
                &mut q,
                &mut trace,
                &maze,
                cell,
                (cell.0, cell.1.saturating_sub(1)),
            );
            Self::explore_cell(&mut q, &mut trace, &maze, cell, (cell.0, cell.1 + 1));
        }

        -1
    }

    #[inline]
    fn explore_cell(
        q: &mut VecDeque<(usize, usize)>,
        trace: &mut HashMap<(usize, usize), (usize, usize)>,
        maze: &Vec<Vec<char>>,
        parent_cell: (usize, usize),
        cell: (usize, usize),
    ) {
        if cell.0 == maze.len() || cell.1 == maze[0].len() || maze[cell.0][cell.1] == '+' {
            return;
        }
        if trace.get(&cell).is_none() {
            trace.insert(cell, parent_cell);
            q.push_back(cell);
        }
    }

    #[inline]
    fn steps_to_entrance(
        trace: &HashMap<(usize, usize), (usize, usize)>,
        entrance: (usize, usize),
        exit: (usize, usize),
    ) -> i32 {
        let mut cell = exit;
        let mut steps = 1;
        while let Some(&c) = trace.get(&cell) {
            if c == entrance {
                return steps;
            }
            steps += 1;
            cell = c;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_exit() {
        assert_eq!(
            4,
            Solution::nearest_exit(
                vec![
                    vec!['+', '.', '+', '+', '+', '+', '+'],
                    vec!['+', '.', '+', '.', '.', '.', '+'],
                    vec!['+', '.', '+', '.', '+', '.', '+'],
                    vec!['+', '.', '.', '.', '+', '.', '+'],
                    vec!['+', '+', '+', '+', '+', '.', '+']
                ],
                vec![3, 2]
            )
        );

        assert_eq!(
            7,
            Solution::nearest_exit(
                vec![
                    ['+', '.', '+', '+', '+', '+', '+'].to_vec(),
                    ['+', '.', '+', '.', '.', '.', '+'].to_vec(),
                    ['+', '.', '+', '.', '+', '.', '+'].to_vec(),
                    ['+', '.', '.', '.', '.', '.', '+'].to_vec(),
                    ['+', '+', '+', '+', '.', '+', '.'].to_vec()
                ],
                vec![0, 1]
            )
        );

        assert_eq!(
            1,
            Solution::nearest_exit(
                vec![
                    vec!['+', '+', '.', '+'],
                    vec!['.', '.', '.', '+'],
                    vec!['+', '+', '+', '.']
                ],
                vec![1, 2]
            )
        );

        assert_eq!(
            2,
            Solution::nearest_exit(
                vec![
                    vec!['+', '+', '+'],
                    vec!['.', '.', '.'],
                    vec!['+', '+', '+']
                ],
                vec![1, 0]
            )
        );

        assert_eq!(-1, Solution::nearest_exit(vec![vec!['.', '+']], vec![0, 0]));
    }
}
