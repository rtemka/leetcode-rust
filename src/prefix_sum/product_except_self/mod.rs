//https://leetcode.com/problems/product-of-array-except-self/description
struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut left = Vec::with_capacity(nums.len());
        let mut p = 1;
        for n in nums.iter() {
            left.push(p);
            p *= n;
        }
        p = 1;
        for (i, n) in left.iter().enumerate().rev() {
            (nums[i], p) = (n * p, p * nums[i]);
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_except_self() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );
        assert_eq!(
            vec![0, 0, 9, 0, 0],
            Solution::product_except_self(vec![-1, 1, 0, -3, 3])
        );
        assert_eq!(vec![0, 0], Solution::product_except_self(vec![0, 0]));
        assert_eq!(
            vec![0, -18, 0],
            Solution::product_except_self(vec![9, 0, -2])
        );
        assert_eq!(vec![0, 0, 0], Solution::product_except_self(vec![0, 4, 0]));
    }
}
