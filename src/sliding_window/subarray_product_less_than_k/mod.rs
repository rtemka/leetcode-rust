// #713
// https://leetcode.com/problems/subarray-product-less-than-k/

struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let (mut lo, mut hi) = (0, 0);
        let mut product = 1;

        while hi < nums.len() {
            product *= nums[hi];

            while product >= k && lo <= hi {
                product /= nums[lo];
                lo += 1;
            }

            result += if hi >= lo { hi - lo + 1 } else { 0 };

            hi += 1;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subarray_product_less_than_k() {
        assert_eq!(
            8,
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100)
        );

        assert_eq!(
            0,
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0)
        );

        assert_eq!(1, Solution::num_subarray_product_less_than_k(vec![1], 2));

        assert_eq!(
            0,
            Solution::num_subarray_product_less_than_k(vec![5, 6, 7], 3)
        );

        assert_eq!(
            8,
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6, 120], 100)
        );
    }
}
