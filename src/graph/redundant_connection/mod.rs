// #684
// https://leetcode.com/problems/redundant-connection/description/

struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dsu = Dsu::new(edges.len());

        for edge in edges {
            dsu.union(edge[0], edge[1]);
        }

        dsu.redundant_edge
    }
}

struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
    redundant_edge: Vec<i32>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            rank: vec![1; n + 1],
            redundant_edge: vec![0; 2],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: i32, y: i32) {
        let root_x = self.find(x as usize);
        let root_y = self.find(y as usize);

        if root_x == root_y {
            (self.redundant_edge[0], self.redundant_edge[1]) = (x, y);
            return;
        }

        let (root, child) = if self.rank[root_x] < self.rank[root_y] {
            (root_y, root_x)
        } else if self.rank[root_x] > self.rank[root_y] {
            (root_x, root_y)
        } else {
            if x <= y {
                (root_x, root_y)
            } else {
                (root_y, root_x)
            }
        };

        self.parent[child] = root;
        self.rank[root] += self.rank[child];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redundant_connection() {
        assert_eq!(
            vec![2, 3],
            Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]])
        );

        assert_eq!(
            vec![1, 4],
            Solution::find_redundant_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ])
        );
    }
}
