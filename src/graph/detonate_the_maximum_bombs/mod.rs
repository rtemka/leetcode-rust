// #2101
// https://leetcode.com/problems/detonate-the-maximum-bombs/description/

struct Solution;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); bombs.len()];

        for (i, b1) in bombs.iter().enumerate() {
            let (x1, y1, r1) = (b1[0] as i64, b1[1] as i64, b1[2] as i64);

            for (j, b2) in bombs.iter().enumerate() {
                if i == j {
                    continue;
                }
                let (x2, y2) = (b2[0] as i64, b2[1] as i64);

                if (x1 - x2).pow(2) + (y1 - y2).pow(2) <= r1.pow(2) {
                    graph[i].push(j);
                }
            }
        }

        let mut max_detonations = 0;
        let mut visited = vec![false; graph.len()];

        for i in 0..bombs.len() {
            max_detonations = max_detonations.max(Self::dfs(&graph, &mut visited, i));
            visited.fill(false);
        }

        max_detonations
    }

    fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, node: usize) -> i32 {
        visited[node] = true;
        let mut count = 1;

        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                count += Self::dfs(graph, visited, neighbor);
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detonate_the_maximum_bombs() {
        assert_eq!(
            2,
            Solution::maximum_detonation(vec![vec![2, 1, 3], vec![6, 1, 4]])
        );

        assert_eq!(
            1,
            Solution::maximum_detonation(vec![vec![1, 1, 5], vec![10, 10, 5]])
        );

        assert_eq!(
            5,
            Solution::maximum_detonation(vec![
                vec![1, 2, 3],
                vec![2, 3, 1],
                vec![3, 4, 2],
                vec![4, 5, 3],
                vec![5, 6, 4]
            ])
        );
    }
}
