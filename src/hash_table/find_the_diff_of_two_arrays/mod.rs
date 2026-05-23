use std::collections::HashSet;

// https://leetcode.com/problems/find-the-difference-of-two-arrays/description
struct Solution;

impl Solution {
    pub fn find_difference_manual(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set1: HashSet<i32> = HashSet::from_iter(nums1.iter().cloned());
        let mut set2: HashSet<i32> = HashSet::from_iter(nums2.iter().cloned());
        for i in nums1.iter() {
            set2.remove(i);
        }
        for i in nums2.iter() {
            set1.remove(i);
        }
        vec![
            set1.into_iter().collect::<Vec<i32>>(),
            set2.into_iter().collect::<Vec<i32>>(),
        ]
    }

    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1: HashSet<i32> = HashSet::from_iter(nums1.iter().cloned());
        let set2: HashSet<i32> = HashSet::from_iter(nums2.iter().cloned());
        vec![Vec::from_iter(&set1 - &set2), Vec::from_iter(&set2 - &set1)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_difference() {
        let mut ans = Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]);
        ans.iter_mut().for_each(|v| v.sort_unstable());
        assert_eq!(vec![vec![1, 3], vec![4, 6]], ans,);

        let mut ans = Solution::find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]);
        ans.iter_mut().for_each(|v| v.sort_unstable());
        assert_eq!(vec![vec![3], vec![]], ans,);
    }
}
