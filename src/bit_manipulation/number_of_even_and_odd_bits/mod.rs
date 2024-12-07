// https://leetcode.com/problems/number-of-even-and-odd-bits/description/
struct Solution;

impl Solution {
    pub fn even_odd_bit(mut n: i32) -> Vec<i32> {
        let mut res = vec![0, 0];
        let mut i = 0;
        while n > 0 {
            if n & 1 == 1 {
                if i % 2 == 0 {
                    res[0] += 1;
                } else {
                    res[1] += 1;
                }
            }
            n >>= 1;
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_odd_bit() {
        assert_eq!(vec![1, 2], Solution::even_odd_bit(50));
        assert_eq!(vec![0, 1], Solution::even_odd_bit(2));
    }
}
