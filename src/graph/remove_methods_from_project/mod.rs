// #3310
// https://leetcode.com/problems/remove-methods-from-project

struct Solution;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: Vec<Vec<i32>> = vec![Vec::new(); n as usize];

        for invocation in &invocations {
            let (from, to) = (invocation[0] as usize, invocation[1]);
            graph[from].push(to);
        }

        let mut visited = vec![false; n as usize];

        Self::dfs(&graph, k as usize, &mut visited);

        let invoked_outside = graph
            .into_iter()
            .enumerate()
            .filter(|&(node, _)| !visited[node])
            .any(|(_, edges)| edges.into_iter().any(|edge| visited[edge as usize]));

        if invoked_outside {
            (0..n).collect()
        } else {
            (0..n).filter(|&x| !visited[x as usize]).collect()
        }
    }

    fn dfs(graph: &[Vec<i32>], root: usize, visited: &mut [bool]) {
        if visited[root] {
            return;
        }

        visited[root] = true;

        for &neigbor in graph[root].iter() {
            Self::dfs(graph, neigbor as usize, visited);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_methods_from_project() {
        assert_eq!(
            vec![0, 1, 2, 3],
            Solution::remaining_methods(4, 1, vec![vec![1, 2], vec![0, 1], vec![3, 2]])
        );

        assert_eq!(
            Vec::<i32>::new(),
            Solution::remaining_methods(3, 2, vec![vec![1, 2], vec![0, 1], vec![2, 0]])
        );
    }
}
