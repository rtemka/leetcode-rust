// #1031
// https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays/description/
// Explanation
// https://algo.monster/liteproblems/1031

struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        // Build prefix sum array where prefixSum[i] = sum of nums[0...i-1]
        let mut prefix_sum = vec![0_i32; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }

        // Choose max between two variants
        i32::max(
            // Case 1: First subarray comes before second subarray
            Self::max_sum(&nums, &prefix_sum, first_len as usize, second_len as usize),
            // Case 2: Second subarray comes before first subarray
            Self::max_sum(&nums, &prefix_sum, second_len as usize, first_len as usize),
        )
    }

    #[inline]
    fn max_sum(nums: &[i32], prefix_sum: &[i32], first_len: usize, second_len: usize) -> i32 {
        // Track the maximum sum of first subarray ending before position i
        let mut first_max_sum = 0;
        // Track total max
        let mut max_sum = 0;

        // To calculate subarray sums quickly, we use prefix sums.
        // With prefix sum array `s`, the sum of elements from index i to j is simply s[j+1] - s[i].
        println!("nums=      {:?}", nums);
        println!("prefix_sum={:?}", prefix_sum);
        println!("bounds={}..{}", first_len, nums.len() - second_len);

        // Iterate through possible positions for the second subarray
        for i in first_len..=nums.len() - second_len {
            let first_sum = prefix_sum[i] - prefix_sum[i - first_len];
            let second_sum = prefix_sum[i + second_len] - prefix_sum[i];

            first_max_sum = i32::max(first_max_sum, first_sum);
            max_sum = i32::max(max_sum, first_max_sum + second_sum);

            println!(
                "first_sum: [{}..{}]={}",
                i - first_len,
                i,
                prefix_sum[i] - prefix_sum[i - first_len]
            );
            println!(
                "secnd_sum: [{}..{}]={}",
                i,
                i + second_len,
                prefix_sum[i + second_len] - prefix_sum[i]
            );

            println!("first_max_sum={},max_sum={}", first_max_sum, max_sum)
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_sum_of_two_non_overlapping_subarrays() {
        assert_eq!(
            31,
            Solution::max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3)
        );

        assert_eq!(
            20,
            Solution::max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2)
        );
    }
}
