// https://leetcode.com/problems/find-pivot-index/description/
struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left_sum = Vec::with_capacity(nums.len());
        let mut prev = 0;
        for &i in nums.iter() {
            left_sum.push(prev);
            prev += i;
        }
        prev = 0;
        let mut res = -1;
        for (i, n) in nums.iter().enumerate().rev() {
            if left_sum[i] == prev {
                res = i as i32;
            }
            prev += n;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pivot_index() {
        assert_eq!(3, Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]));
        assert_eq!(-1, Solution::pivot_index(vec![1, 2, 3]));
        assert_eq!(0, Solution::pivot_index(vec![2, 1, -1]));
    }
}
