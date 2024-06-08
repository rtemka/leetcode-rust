use std::collections::HashMap;

// https://leetcode.com/problems/intersection-of-two-arrays-ii/description/
struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (short, long) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(short.len());
        for i in short.iter() {
            map.entry(*i).and_modify(|n| *n += 1).or_insert(1);
        }
        let mut v = Vec::new();
        for i in long.iter() {
            if let Some(n) = map.get_mut(i) {
                if *n > 0 {
                    v.push(*i);
                    *n -= 1;
                }
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
        assert_eq!(
            vec![2, 2],
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2])
        );

        let mut expected = vec![4, 9];
        let mut got = Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        expected.sort_unstable();
        got.sort_unstable();
        assert_eq!(expected, got);
    }
}
