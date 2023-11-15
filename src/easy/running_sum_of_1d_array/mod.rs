/// https://leetcode.com/problems/richest-customer-wealth/description/
struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let mut prev = 0;
        for i in nums.into_iter() {
            prev += i;
            result.push(prev);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use log;

    /// RUST_LOG=debug cargo t running_sum
    #[test]
    fn running_sum() {
        env_logger::init();
        log::debug!("something to print in tests");

        assert_eq!(vec![1, 3, 6], Solution::running_sum(vec![1, 2, 3]));

        assert_eq!(
            vec![3, 4, 6, 16, 17],
            Solution::running_sum(vec![3, 1, 2, 10, 1])
        );
    }
}
