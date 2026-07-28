// #2492
// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities

struct Solution;

#[derive(Debug)]
struct Dsu {
    parent: Vec<usize>,
    rank: Vec<i32>,
    min_score: Vec<i32>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..=n).collect(),
            rank: vec![1; n + 1],
            min_score: vec![i32::MAX; n + 1],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = Self::find(self, self.parent[x])
        }
        self.parent[x]
    }

    fn union(&mut self, x: i32, y: i32, weight: i32) -> bool {
        let root_x = Self::find(self, x as usize);
        let root_y = Self::find(self, y as usize);

        let min_score = i32::min(
            weight,
            i32::min(self.min_score[root_x], self.min_score[root_y]),
        );

        println!("x={x}-{root_x}, y={y}-{root_y}, weight={weight}");

        if root_x == root_y {
            self.min_score[root_x] = min_score;
            return false;
        }

        let (root, child) = if self.rank[root_x] < self.rank[root_y] {
            (root_y, root_x)
        } else {
            (root_x, root_y)
        };

        self.parent[child] = root;
        self.rank[root] += self.rank[child];
        self.min_score[root] = min_score;

        println!(
            "root={root}-{}, child={child}-{}",
            self.min_score[root], self.min_score[child]
        );

        true
    }
}

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut dsu = Dsu::new(n as usize);

        for road in roads {
            let (from, to, weight) = (road[0], road[1], road[2]);
            dsu.union(from, to, weight);
        }

        println!("{:?}", dsu);

        let root_for_city_1 = dsu.find(1);

        dsu.min_score[root_for_city_1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_score_of_a_path_between_two_cities() {
        assert_eq!(
            5,
            Solution::min_score(
                4,
                vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]]
            )
        );
    }
}
