struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        // This is because a power of two in binary representation has only one bit
        // set to 1, and subtracting 1 from a power of two results in
        // a number with all lower bits set to 1. For example, 8 in
        // binary is 1000, and 7 in binary is 0111.
        //
        // When we perform the bitwise AND operation between 8 (1000) and 7 (0111), the result is 0.
        if n > 0 {
            n & n - 1 == 0
        } else {
            // all negatives and zero is not the power of 2.
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_power_of_two() {
        assert!(Solution::is_power_of_two(4));
        assert!(Solution::is_power_of_two(16));
        assert!(!Solution::is_power_of_two(-16));
        assert!(!Solution::is_power_of_two(7));
        assert!(!Solution::is_power_of_two(0));
    }
}
