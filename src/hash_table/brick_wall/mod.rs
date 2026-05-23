// https://leetcode.com/problems/brick-wall/description/
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let wall_len = wall.len();
        // add all possible heights in each wall row
        for row in wall {
            let mut sum = 0i32;
            // skip the last brick
            // to exclude the right side of the wall
            // which is max sum for each wall row
            let bricks_count = row.len() - 1;
            for brick in row.into_iter().take(bricks_count) {
                sum = sum.saturating_add(brick);
                map.entry(sum).and_modify(|count| *count += 1).or_insert(1);
            }
        }
        // find the most frequent height in the wall
        if let Some((_, gaped_bricks)) = map.into_iter().max_by(|(_, v1), (_, v2)| v1.cmp(v2)) {
            // if we subtract the bricks with gap from total wall rows
            // we get the number of crossed bricks
            (wall_len - gaped_bricks) as i32
        } else {
            // for example all rows of wall consist of just one brick
            wall_len as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brick_wall() {
        assert_eq!(
            2,
            Solution::least_bricks(vec![
                vec![1, 2, 2, 1],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 4],
                vec![3, 1, 2],
                vec![1, 3, 1, 1],
            ])
        );

        assert_eq!(3, Solution::least_bricks(vec![vec![1], vec![1], vec![1],]));

        assert_eq!(
            2,
            Solution::least_bricks(vec![vec![1, 1], vec![2], vec![2],])
        );

        assert_eq!(
            1,
            Solution::least_bricks(vec![vec![1, 1], vec![2], vec![1, 1],])
        );
    }
}
