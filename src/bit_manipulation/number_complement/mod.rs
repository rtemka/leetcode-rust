// https://leetcode.com/problems/number-complement/description/
struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut complement = 0;
        let mut found_set_bit = false;
        // Iterate from the 30th bit to the 0th bit (31st bit is not considered for a signed integer)
        for i in (0..31).rev() {
            // Check if the ith bit is set or not
            let bit = num & (1 << i);

            // Skip leading zeroes and look for the first 1
            if !found_set_bit && bit == 0 {
                continue;
            }

            // If the current bit is 0, set the corresponding bit in complement
            found_set_bit = true;
            if bit == 0 {
                complement |= 1 << i;
            }
        }
        complement
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_complement() {
        assert_eq!(2, Solution::find_complement(5));
        assert_eq!(0, Solution::find_complement(1));
    }
}
