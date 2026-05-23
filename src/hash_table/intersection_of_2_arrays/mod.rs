use std::collections::HashMap;

// https://leetcode.com/problems/intersection-of-two-arrays/description/
struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            Self::intersection_helper(nums1, nums2)
        } else {
            Self::intersection_helper(nums2, nums1)
        }
    }

    #[inline]
    fn intersection_helper(to_search: Vec<i32>, searchable: Vec<i32>) -> Vec<i32> {
        let mut map = searchable
            .iter()
            .map(|x| (x, None))
            .collect::<HashMap<&i32, Option<()>>>();
        let mut size = 0;
        for i in &to_search {
            map.entry(i).and_modify(|o| {
                size += 1;
                *o = Some(())
            });
        }
        let mut v = Vec::with_capacity(size);
        for (&k, o) in map {
            if o.is_some() {
                v.push(k);
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_of_two_arr() {
        assert_eq!(
            vec![2],
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2])
        );

        let mut ans = Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        ans.sort_unstable();
        assert_eq!(vec![4, 9], ans,);

        let v: Vec<i32> = vec![];
        let mut ans =
            Solution::intersection(vec![4, 9, 5], vec![1, 2, 3, 6, 45, 66, 77, 88, 99, 100]);
        ans.sort_unstable();
        assert_eq!(v, ans,);
    }
}
