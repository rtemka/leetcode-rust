// https://leetcode.com/problems/bitwise-xor-of-all-pairings/description
struct Solution;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let is_even1 = nums1.len() % 2 == 0;
        let is_even2 = nums2.len() % 2 == 0;
        match (is_even1, is_even2) {
            // x ^ 0 = x;
            // x ^ x = 0;
            //
            // If both vectors are even, then all nums in nums1 will be eliminated by XORing
            // and all nums in nums2 will be eliminated by XORing.
            (true, true) => 0,
            // If one vector is even and other one is not, then all nums from nums1
            // will appear once in resulting XOR, because we multiply nums1*nums2.
            (true, false) => nums1.into_iter().fold(0, |acc, x| acc ^ x),
            // Same here.
            (false, true) => nums2.into_iter().fold(0, |acc, x| acc ^ x),
            // If both odd then we just XOR all from nums1 and nums2.
            (false, false) => {
                let xor = nums1.into_iter().fold(0, |acc, x| acc ^ x);
                nums2.into_iter().fold(0, |acc, x| acc ^ (x ^ xor))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_all_pairings() {
        assert_eq!(13, Solution::xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]));
        assert_eq!(13, Solution::xor_all_nums(vec![10, 2, 5, 0], vec![2, 1, 3]));
        assert_eq!(0, Solution::xor_all_nums(vec![1, 2], vec![3, 4]));
        assert_eq!(7, Solution::xor_all_nums(vec![1, 2, 3], vec![4, 5, 6]));
    }
}
