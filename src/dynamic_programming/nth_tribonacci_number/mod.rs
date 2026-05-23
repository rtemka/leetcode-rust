// https://leetcode.com/problems/n-th-tribonacci-number/description
struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut sequence: [i32; 38] = [0; 38];
        (sequence[0], sequence[1], sequence[2]) = (0, 1, 1);
        let n = n as usize;
        for i in 3..=n {
            sequence[i] = sequence[i - 3] + sequence[i - 2] + sequence[i - 1];
        }
        sequence[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tribonacci() {
        assert_eq!(4, Solution::tribonacci(4));
        assert_eq!(1389537, Solution::tribonacci(25));
    }
}
