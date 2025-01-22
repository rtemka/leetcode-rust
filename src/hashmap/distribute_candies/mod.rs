use std::collections::HashSet;

// https://leetcode.com/problems/distribute-candies/description/
struct Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let max_candies = candy_type.len() / 2;
        candy_type
            .into_iter()
            .collect::<HashSet<i32>>()
            .len()
            .min(max_candies) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distribute_candies() {
        assert_eq!(3, Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]));
        assert_eq!(2, Solution::distribute_candies(vec![1, 1, 2, 3]));
        assert_eq!(1, Solution::distribute_candies(vec![6, 6, 6, 6]));
    }
}
