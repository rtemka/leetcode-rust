// #3876
// https://leetcode.com/problems/construct-uniform-parity-array-ii/description

struct Solution;

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let (mut min_even, mut min_odd) = (i32::MAX, i32::MAX);
        for x in nums1 {
            if x % 2 == 0 {
                min_even = i32::min(min_even, x);
            } else {
                min_odd = i32::min(min_odd, x);
            }
        }

        min_odd < min_even || min_odd == i32::MAX
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_uniform_parity_array_2() {
        assert!(Solution::uniform_array(vec![1, 4, 7]));
    }
}
