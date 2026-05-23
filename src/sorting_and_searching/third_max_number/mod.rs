// https://leetcode.com/problems/third-maximum-number/description/
struct Solution {}

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut max = nums[0]; // 1 <= nums.length <= 10 ^ 4;
        for i in nums.iter() {
            if *i > max {
                max = *i;
            }
        }
        let mut sec: Option<i32> = None;
        for i in nums.iter() {
            match sec {
                Some(s) if *i < max && *i > s => sec = Some(*i),
                None if *i < max => sec = Some(*i),
                _ => continue,
            }
        }
        if sec.is_none() {
            return max;
        }
        let s = sec.unwrap();
        let mut third: Option<i32> = None;
        for i in nums.iter() {
            match third {
                Some(t) if *i < s && *i > t => third = Some(*i),
                None if *i < s => third = Some(*i),
                _ => continue,
            }
        }
        if let Some(t) = third {
            t
        } else {
            max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn third_max() {
        assert_eq!(1, Solution::third_max(vec![3, 2, 1]));
        assert_eq!(2, Solution::third_max(vec![1, 2]));
        assert_eq!(1, Solution::third_max(vec![2, 2, 3, 1]));
    }
}
