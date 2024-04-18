// https://leetcode.com/problems/minimum-size-subarray-sum/description/
struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, 0);
        let mut min_len = 0;
        let mut sum = nums[hi]; // constraints: starts from 1;
        let mut up = false;
        while hi < nums.len() {
            // println!("{:?}->min_len:{}\tlo:{}|hi{}", nums, min_len, lo, hi);
            if nums[hi] >= target {
                return 1;
            }
            if up {
                sum += nums[hi];
            }
            if sum >= target {
                let cur_len = hi - lo + 1;
                if min_len == 0 || cur_len < min_len {
                    min_len = cur_len;
                }
                sum -= nums[lo];
                lo += 1;
                up = false;
            } else {
                up = true;
                hi += 1;
            }
        }
        min_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_sub_array_len() {
        assert_eq!(2, Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
        assert_eq!(1, Solution::min_sub_array_len(4, vec![1, 4, 4]));
        assert_eq!(3, Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]));
        assert_eq!(
            0,
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1])
        );
    }
}
