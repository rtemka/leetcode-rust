// https://leetcode.com/problems/single-number/description/
struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // XOR
        // x ^ x = 0
        // x ^ 0 = x
        unsafe { nums.into_iter().reduce(|acc, x| acc ^ x).unwrap_unchecked() }

        // // manual loop
        // let mut x = 0;
        // for i in nums {
        //     x ^= i;
        // }
        // x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
        assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
        assert_eq!(1, Solution::single_number(vec![1]));
    }
}
