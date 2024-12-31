use std::cmp::Ordering;

// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/description/
struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_unstable_by(
            |a, b| match Self::count_ones(*a).cmp(&Self::count_ones(*b)) {
                Ordering::Equal => a.cmp(b),
                o => o,
            },
        );
        arr
    }

    #[inline]
    fn count_ones(mut n: i32) -> i32 {
        let mut count = 0;
        while n > 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_by_bits() {
        assert_eq!(
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7],
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
        assert_eq!(
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1])
        );
    }
}
