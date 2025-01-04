// https://leetcode.com/problems/xor-operation-in-an-array/description/
struct Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut res = start + 2 * 0;
        for i in 1..n {
            res ^= start + 2 * i;
            // println!("start={};res={}", start + 2 * i, res);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_operation() {
        assert_eq!(8, Solution::xor_operation(5, 0));
        assert_eq!(8, Solution::xor_operation(4, 3));
    }
}
