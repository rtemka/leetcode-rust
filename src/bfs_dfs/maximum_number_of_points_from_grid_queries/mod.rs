// https://leetcode.com/problems/maximum-number-of-points-from-grid-queries/description/
struct Solution;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, mut queries: Vec<i32>) -> Vec<i32> {
        // Sort but preserve order of queries.
        let mut q_index: Vec<usize> = queries.iter().enumerate().map(|(i, _)| i).collect();
        q_index.sort_unstable_by(|a, b| queries[*a].cmp(&queries[*b]));
        // Set for processed cells.
        let mut marked: HashSet<(usize, usize)> = HashSet::new();
        // BFS Min-heap queue.
        let mut queue: BinaryHeap<Reverse<(i32, (usize, usize))>> = BinaryHeap::new();
        // Add starting cell.
        queue.push(Reverse((grid[0][0], (0, 0))));
        marked.insert((0, 0));
        let mut count = 0;
        for i in 0..queries.len() {
            let i = q_index[i];
            count += Self::bfs(&mut marked, &mut queue, &grid, queries[i]);
            queries[i] = count;
        }
        queries
    }

    #[inline]
    fn bfs(
        marked: &mut HashSet<(usize, usize)>,
        q: &mut BinaryHeap<Reverse<(i32, (usize, usize))>>,
        grid: &Vec<Vec<i32>>,
        query: i32,
    ) -> i32 {
        let mut count = 0;
        while let Some(Reverse((v, _))) = q.peek() {
            if *v >= query {
                break;
            }
            if let Some(Reverse(cell)) = q.pop() {
                Self::add_neighbors(cell.1, marked, q, grid);
            }
            count += 1;
        }
        count
    }

    #[inline]
    fn add_neighbors(
        coord: (usize, usize),
        marked: &mut HashSet<(usize, usize)>,
        q: &mut BinaryHeap<Reverse<(i32, (usize, usize))>>,
        grid: &Vec<Vec<i32>>,
    ) {
        // Up
        Self::enqueue((coord.0.saturating_sub(1), coord.1), marked, q, grid);
        // Down
        Self::enqueue((coord.0 + 1, coord.1), marked, q, grid);
        // Left
        Self::enqueue((coord.0, coord.1.saturating_sub(1)), marked, q, grid);
        // Right
        Self::enqueue((coord.0, coord.1 + 1), marked, q, grid);
    }

    #[inline]
    fn enqueue(
        coord: (usize, usize),
        marked: &mut HashSet<(usize, usize)>,
        q: &mut BinaryHeap<Reverse<(i32, (usize, usize))>>,
        grid: &Vec<Vec<i32>>,
    ) {
        if coord.0 < grid.len() && coord.1 < grid[0].len() && !marked.contains(&coord) {
            let cell = (grid[coord.0][coord.1], coord);
            q.push(Reverse(cell));
            marked.insert(coord);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_number_of_points_from_grid_queries() {
        assert_eq!(
            vec![5, 8, 1],
            Solution::max_points(
                vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
                vec![5, 6, 2]
            )
        );

        assert_eq!(
            vec![0],
            Solution::max_points(vec![vec![5, 2, 1], vec![1, 1, 2]], vec![3])
        );
    }
}
