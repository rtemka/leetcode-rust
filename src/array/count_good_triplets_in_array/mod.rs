// #2179
// https://leetcode.com/problems/count-good-triplets-in-an-array/description
struct Solution;

impl Solution {
    pub fn count_good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_good_triplets_in_array() {
        assert_eq!(
            1,
            Solution::count_good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3])
        );
    }
}
