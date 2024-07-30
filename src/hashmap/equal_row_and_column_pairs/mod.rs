use std::collections::HashMap;

// https://leetcode.com/problems/equal-row-and-column-pairs/description
struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<Vec<i32>, i32> = HashMap::with_capacity(grid[0].len());
        for v in grid.iter() {
            map.entry(v.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut v = Vec::with_capacity(grid.len());
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                v.push(grid[j][i]);
            }
            if let Some(c) = map.get(&v) {
                count += c;
            }
            v.truncate(0);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_pairs() {
        assert_eq!(
            1,
            Solution::equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]])
        );
        assert_eq!(
            3,
            Solution::equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ])
        );
        assert_eq!(
            3,
            Solution::equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 4],
                vec![2, 4, 2, 2],
                vec![2, 5, 2, 2]
            ])
        );
        assert_eq!(4, Solution::equal_pairs(vec![vec![13, 13], vec![13, 13],]));
    }
}
