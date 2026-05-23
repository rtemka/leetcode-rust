// https://leetcode.com/problems/maximum-average-subarray-i/description
struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let (mut lo, mut hi, k) = (0, 0, k as usize);
        let mut avg = f64::MIN;
        let mut sum = 0;
        while hi < nums.len() {
            sum += nums[hi];
            if hi - lo < k - 1 {
                hi += 1;
            } else {
                avg = avg.max(sum as f64 / (hi - lo + 1) as f64);
                sum -= nums[lo];
                (lo, hi) = (lo + 1, hi + 1);
            }
            println!("lo={};hi={}\tsum={};avg={}", lo, hi, sum, avg);
        }
        avg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_average() {
        assert_eq!(
            12.75,
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4)
        );
        assert_eq!(2.80, Solution::find_max_average(vec![4, 0, 4, 3, 3], 5));
        assert_eq!(5.0, Solution::find_max_average(vec![5], 1));
        assert_eq!(-1.0, Solution::find_max_average(vec![-1], 1));
        assert!(
            (526.37288
                - Solution::find_max_average(
                    vec![
                        4433, -7832, -5068, 4009, 2830, 6544, -6119, -7126, -780, -4254, -8249,
                        -9168, 9492, 402, 5789, 6808, 8953, 5810, -7353, 7933, 4766, 5182, -3230,
                        -1989, 5786, 6922, -4646, 4415, -9906, 807, -6373, 3370, 2604, 8751, -9173,
                        -2668, -6876, 9500, 3465, -1900, 4134, -1758, -1453, -5201, -9825, 4469,
                        -1999, -1108, 1836, 3923, 6796, -5252, 9863, -5997, -3251, 9596, -3404,
                        -540, 2826, -1737, 3341, -3623, -9885, 2603, -5782, 8174, 2710, 6504,
                        -4128
                    ],
                    59
                ))
                < 1.0e-6,
        );
    }
}
