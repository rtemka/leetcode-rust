// #2812
// https://leetcode.com/problems/find-the-safest-path-in-a-grid/description
// See also: #3286

#![allow(clippy::needless_range_loop)]

use std::collections::VecDeque;

struct Solution;

const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut queue = VecDeque::with_capacity(n * m);

        // Расставим по матрице расстояние до ближайших "воров".
        Self::set_manhattan_distance(&mut grid, &mut queue);

        // Отсечес случай когда добраться изначально невозможно.
        if grid[0][0] < 0 || grid[m - 1][n - 1] < 0 {
            return 0;
        }

        // Инициализируем расстоянием от первой клетки.
        let mut min_distance_to_thief = grid[0][0];

        // Делай значения отрицательными, мы помечаем таким образом клетки, как visited.
        grid[0][0] = -grid[0][0];
        queue.push_back((0, 0));

        while let Some(cell) = queue.pop_front() {
            // Фиксируем минимальную дистанцию до сих пор, беря аболютное значение |grid[x][y]|
            min_distance_to_thief = min_distance_to_thief.min(grid[cell.0][cell.1].abs());

            // Если дошли до финиша, возвращаем минимальную дистанцию до ближайшего "вора"
            if cell == (m - 1, n - 1) {
                return min_distance_to_thief;
            }

            for dir in DIRS {
                // Обрабатываем только еще непосещенных соседей
                if let Some(next) =
                    Self::get_neighbor(&grid, (cell.0 as isize + dir.0, cell.1 as isize + dir.1))
                    && grid[next.0][next.1] > 0
                {
                    // Если дистанция хуже чем сейчас, то в конец очереди, иначе в начало
                    if grid[next.0][next.1] < min_distance_to_thief {
                        queue.push_back(next);
                    } else {
                        queue.push_front(next);
                    }

                    // "Помечаем" таким образом клетки как посещенные уже.
                    grid[next.0][next.1] = -grid[next.0][next.1];
                }
            }
        }

        0
    }

    fn set_manhattan_distance(grid: &mut [Vec<i32>], queue: &mut VecDeque<(usize, usize)>) {
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] == 1 {
                    grid[r][c] = -1;
                    queue.push_back((r, c));
                }
            }
        }

        while let Some(cell) = queue.pop_front() {
            for dir in DIRS {
                if let Some(next) =
                    Self::get_neighbor(grid, (cell.0 as isize + dir.0, cell.1 as isize + dir.1))
                    && grid[next.0][next.1] == 0
                {
                    grid[next.0][next.1] = i32::max(0, grid[cell.0][cell.1]) + 1;
                    queue.push_back(next);
                }
            }
        }
    }

    #[inline]
    fn get_neighbor(grid: &[Vec<i32>], cell: (isize, isize)) -> Option<(usize, usize)> {
        if cell.0 < 0
            || cell.1 < 0
            || cell.0 as usize == grid.len()
            || cell.1 as usize == grid[0].len()
        {
            None
        } else {
            Some((cell.0 as usize, cell.1 as usize))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_the_safest_path_in_a_grid() {
        assert_eq!(
            Solution::maximum_safeness_factor(vec![vec![0, 0, 1], vec![0, 0, 1], vec![0, 0, 0]]),
            1
        );

        assert_eq!(
            Solution::maximum_safeness_factor(vec![vec![0, 1, 1], vec![1, 1, 1], vec![1, 1, 0],]),
            0
        );

        assert_eq!(
            Solution::maximum_safeness_factor(vec![
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 0, 0, 0]
            ]),
            2
        );

        assert_eq!(
            Solution::maximum_safeness_factor(vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]]),
            2
        );
    }
}
