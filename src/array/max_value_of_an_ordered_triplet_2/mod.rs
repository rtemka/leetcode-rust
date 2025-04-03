// https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-ii/description
struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut max = 0;
        let mut diff = 0;
        let mut val = 0;
        for n in nums {
            max = i64::max(max, diff * n as i64);
            val = i64::max(val, n as i64);
            diff = i64::max(diff, val - n as i64);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_value_of_an_ordered_triplet_2() {
        assert_eq!(77, Solution::maximum_triplet_value(vec![12, 6, 1, 2, 7]));
        assert_eq!(133, Solution::maximum_triplet_value(vec![1, 10, 3, 4, 19]));
        assert_eq!(0, Solution::maximum_triplet_value(vec![1, 2, 3]));
        // assert_eq!(0, Solution::maximum_triplet_value(vec![-3, -2, 1, 10]));
        assert_eq!(
            9999990000000,
            Solution::maximum_triplet_value(vec![1000000, 1, 10000000])
        );
    }
}
