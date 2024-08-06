use std::collections::HashMap;

// https://leetcode.com/problems/max-number-of-k-sum-pairs/description
struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len() / 2);
        for x in nums.iter() {
            if let Some(y) = map.get_mut(&(k - x)) {
                if *y == 1 {
                    map.remove(&(k - x));
                } else {
                    *y -= 1;
                }
                count += 1;
            } else {
                map.entry(*x).and_modify(|v| *v += 1).or_insert(1);
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_operations() {
        assert_eq!(2, Solution::max_operations(vec![1, 2, 3, 4], 5));
        assert_eq!(1, Solution::max_operations(vec![3, 1, 3, 4, 3], 6));
        assert_eq!(0, Solution::max_operations(vec![1], 1));
        assert_eq!(0, Solution::max_operations(vec![1, 5, 5, 5, 5], 8));
        assert_eq!(3, Solution::max_operations(vec![1, 5, 5, 5, 5, 5, 5], 10));
        assert_eq!(
            4,
            Solution::max_operations(
                vec![2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2],
                3
            )
        );
    }
}
