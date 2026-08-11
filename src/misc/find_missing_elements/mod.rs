// #3731
// https://leetcode.com/problems/find-missing-elements/description

struct Solution;

impl Solution {
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
        let mut missing = Vec::new();

        nums.sort_unstable();

        for i in 1..nums.len() {
            if nums[i] - nums[i - 1] > 1 {
                missing.extend(nums[i - 1] + 1..nums[i]);
            }
        }

        missing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_missing_elements() {
        assert_eq!(vec![3], Solution::find_missing_elements(vec![1, 4, 2, 5]));

        assert_eq!(
            Vec::<i32>::new(),
            Solution::find_missing_elements(vec![7, 8, 6, 9])
        );

        assert_eq!(vec![2, 3, 4], Solution::find_missing_elements(vec![5, 1]));
    }
}
