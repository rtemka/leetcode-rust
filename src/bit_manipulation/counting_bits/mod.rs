// https://leetcode.com/problems/counting-bits/description
struct Solution;

// Brian Kernigan apporach (O(logN)): https://stackoverflow.com/a/12381102

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(n as usize + 1);
        for i in 0..=n {
            ans.push(Self::count_set_bits(i));
        }
        ans
    }

    #[inline]
    const fn count_set_bits(mut n: i32) -> i32 {
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
    fn count_bits() {
        assert_eq!(vec![0, 1, 1], Solution::count_bits(2));
    }
}
