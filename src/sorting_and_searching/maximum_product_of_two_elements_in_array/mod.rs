// #1464
// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        // Constraints:
        // 2 <= nums.length <= 500
        // 1 <= nums[i] <= 10^3

        let mut one = i32::max(nums[0], nums[1]);
        let mut two = i32::min(nums[0], nums[1]);

        for &n in nums.iter().skip(2) {
            if n > one {
                two = one;
                one = n;
            } else if n > two {
                two = n;
            }
        }

        (one - 1) * (two - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_product_of_two_elements_in_array() {
        assert_eq!(12, Solution::max_product(vec![3, 4, 5, 2]));
        assert_eq!(16, Solution::max_product(vec![1, 5, 4, 5]));
        assert_eq!(12, Solution::max_product(vec![3, 7]));
    }
}
