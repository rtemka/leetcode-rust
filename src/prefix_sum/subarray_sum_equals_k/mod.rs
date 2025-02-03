// https://leetcode.com/problems/subarray-sum-equals-k/description/
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cur_sum = 0i32;
        let mut count = 0;
        let mut prefix_sum: HashMap<i32, i32> = HashMap::with_capacity(nums.len() + 1);
        prefix_sum.insert(0, 1);
        for val in nums {
            cur_sum += val;
            count += prefix_sum.get(&(cur_sum - k)).unwrap_or(&0);
            prefix_sum
                .entry(cur_sum)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subarray_sum() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
        assert_eq!(2, Solution::subarray_sum(vec![1, 2, 3], 3));
    }
}
