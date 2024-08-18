// https://leetcode.com/problems/ugly-number-ii/description
struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut two = vec![2];
        let mut three = vec![3];
        let mut five = vec![5];
        for _ in 0..n {}
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nth_ugly_number() {
        assert_eq!(12, Solution::nth_ugly_number(10));
    }
}
