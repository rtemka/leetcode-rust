// https://leetcode.com/problems/count-the-number-of-fair-pairs/description
struct Solution;

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let mut count = 0;
        for i in 0..nums.len() {
            let lo = (&nums[i + 1..]).partition_point(|&x| x < lower - nums[i]);
            let hi = (&nums[i + 1..]).partition_point(|&x| x <= upper - nums[i]);
            count += (i + hi) as i64 - (i + lo) as i64;
        }
        count
    }

    pub fn count_fair_pairs_classic(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let mut count = 0;
        for i in 0..nums.len() {
            let low = Self::lower_bound(&nums, i + 1, nums.len() - 1, lower - nums[i]);
            let high = Self::lower_bound(&nums, i + 1, nums.len() - 1, upper - nums[i] + 1);
            count += high - low
        }

        count
    }

    fn lower_bound(nums: &[i32], mut low: usize, mut high: usize, element: i32) -> i64 {
        while low <= high {
            let mid = low + ((high - low) / 2);
            if nums[mid] >= element {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        low as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_the_number_of_fair_pairs() {
        assert_eq!(6, Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6));
        assert_eq!(1, Solution::count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11));
        assert_eq!(
            4,
            Solution::count_fair_pairs(vec![-10, -8, -6, -4, -2, -1], -7, -2)
        );
        assert_eq!(15, Solution::count_fair_pairs(vec![0, 0, 0, 0, 0, 0], 0, 0));
        assert_eq!(
            22,
            Solution::count_fair_pairs(
                vec![10, 9, 7, 5, 1000000000, -1000000000, 100, 200],
                -1000000000,
                1000000000
            )
        );
    }
}
