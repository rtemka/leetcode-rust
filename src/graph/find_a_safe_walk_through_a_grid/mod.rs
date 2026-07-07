// #3286
// https://leetcode.com/problems/find-a-safe-walk-through-a-grid/description

struct Solution;

use std::collections::VecDeque;

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let (m, n) = (grid.len(), grid[0].len());

        // Инициализируем матрицу максимальным уроном
        let mut min_health_lost = vec![vec![i32::MAX; n]; m];
        let mut queue = VecDeque::new();

        // В обычном BFS без весов мы используем HashSet или Vec<Vec<bool>> visited, чтобы не ходить по кругу.
        // Как только клетка посещена, мы в нее больше не заходим.
        // В этой задаче у ребер есть веса (0 или 1). Если мы будем использовать обычный visited,
        // мы можем прийти в клетку сначала длинным, но «дорогим» путем (потратив много здоровья),
        // пометить ее как посещенную, а затем обнаружить более «дешевый» путь.
        // Если клетка уже в visited, мы проигнорируем лучший путь, и алгоритм выдаст неверный ответ.
        // Матрица min_health_lost хранит наименьший урон, с которым мы смогли добраться до конкретной клетки на данный момент.
        min_health_lost[0][0] = grid[0][0];
        queue.push_back((0, 0));

        while let Some(cell) = queue.pop_front() {
            let damage = min_health_lost[cell.0][cell.1];

            // Если дошли до финиша, проверяем, осталось ли здоровье
            if cell == (m - 1, n - 1) {
                return health - damage > 0;
            }

            for dir in DIRS {
                if let Some(neighbour) =
                    Self::get_neighbor(&grid, (cell.0 as isize + dir.0, cell.1 as isize + dir.1))
                {
                    let next_damage = damage + grid[neighbour.0][neighbour.1];

                    // Критерий обновления пути — это релаксация ребер, как в алгоритме Дейкстры:
                    // Если новый путь в клетку (r, c) наносит меньше урона, чем все предыдущие варианты,
                    // мы заходим в нее снова, обновляем значение и добавляем в очередь.
                    // Если новый путь хуже или такой же, мы просто игнорируем его.
                    // Это предотвращает бесконечное хождение по сетке.
                    // Заходим в клетку только если нашли более безопасный путь.
                    if next_damage < min_health_lost[neighbour.0][neighbour.1] {
                        min_health_lost[neighbour.0][neighbour.1] = next_damage;

                        if grid[neighbour.0][neighbour.1] == 0 {
                            queue.push_front(neighbour); // Бесплатный шаг — в начало
                        } else {
                            queue.push_back(neighbour); // Платный шаг — в конец
                        }
                    }
                }
            }
        }

        false
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
    fn find_a_safe_walk_through_a_grid() {
        assert!(!Solution::find_safe_walk(vec![vec![1, 1, 1, 1],], 4));

        assert!(Solution::find_safe_walk(
            vec![
                vec![1, 1, 1, 1],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 0, 1, 1]
            ],
            6
        ));

        assert!(Solution::find_safe_walk(
            vec![
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 0, 1, 0]
            ],
            1
        ));

        assert!(!Solution::find_safe_walk(
            vec![
                vec![0, 1, 1, 0, 0, 0],
                vec![1, 0, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 0, 1],
                vec![0, 0, 1, 0, 1, 0]
            ],
            3
        ));

        assert!(Solution::find_safe_walk(
            vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1],],
            5
        ));
    }
}
