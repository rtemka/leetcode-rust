// https://leetcode.com/problems/binary-prefix-divisible-by-5/description/
struct Solution;

impl Solution {
    // wrong answer, cause left shift of negative vals
    pub fn prefixes_div_by5_wa(nums: Vec<i32>) -> Vec<bool> {
        let mut n = 0;
        let mut res = Vec::with_capacity(nums.len());
        for bit in nums {
            n = (n << 1) | bit;
            res.push(n % 5 == 0);
        }
        res
    }

    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut n = 0;
        let mut res = Vec::with_capacity(nums.len());
        for bit in nums {
            n = ((n * 2) + bit) % 5;
            res.push(n == 0);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_div_by_5() {
        assert_eq!(
            vec![
                false, false, true, false, false, false, false, false, false, false, true, true,
                true, true, true, true, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, true, false, false, false, true,
                false, false, true, false, false, true, true, true, true, true, true, true, false,
                false, true, false, false, false, false, true, true
            ],
            Solution::prefixes_div_by5(vec![
                1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0,
                1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 0,
                0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0
            ])
        );
        assert_eq!(
            vec![
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, true, false, false, true, true,
                true, true, false
            ],
            Solution::prefixes_div_by5(vec![
                1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0,
                0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1
            ])
        );
        assert_eq!(
            vec![true, false, false],
            Solution::prefixes_div_by5(vec![0, 1, 1])
        );
        assert_eq!(
            vec![false, false, false],
            Solution::prefixes_div_by5(vec![1, 1, 1])
        );
        assert_eq!(
            vec![false, false, true, true, false, false, true, true],
            Solution::prefixes_div_by5(vec![1, 0, 1, 0, 1, 0, 1, 0])
        );
    }
}
