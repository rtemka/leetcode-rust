struct Solution {}
// https://leetcode.com/problems/missing-number/
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (nums.len() * (nums.len() + 1) / 2) as i32 - nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_number() {
        assert_eq!(8, Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
        assert_eq!(2, Solution::missing_number(vec![0, 1]));
        assert_eq!(2, Solution::missing_number(vec![0, 1, 3]));
    }
}
