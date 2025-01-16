struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        // 1. create a mask of the all powers of 4.
        // 2. OR mask and n. If it possibly power of 4 then the result of OR will be equal to mask.
        // 3. Then we must check that n have only 1 bit set. For this we check n&n-1 == 0.
        let mask = 0b01010101010101010101010101010101;
        n > 0 && mask == n | mask && n & n - 1 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_power_of_four() {
        assert!(!Solution::is_power_of_four(12));
        assert!(!Solution::is_power_of_four(3));
        assert!(!Solution::is_power_of_four(2));
        assert!(!Solution::is_power_of_four(-2147483648));
        assert!(Solution::is_power_of_four(16));
        assert!(Solution::is_power_of_four(1));
        assert!(!Solution::is_power_of_four(5));
    }
}
