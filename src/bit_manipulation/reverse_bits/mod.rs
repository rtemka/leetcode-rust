// https://leetcode.com/problems/reverse-bits/description/
struct Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        // Initialize result as 0. This will hold the reversed bits.
        let mut result = 0;
        // Loop over the 32 bits of the integer.
        for i in 0..32 {
            // Extract the least significant bit (LSB) using the bitwise AND operation (&)
            // and shift it to its reversed position. Then, use the bitwise OR operation (|)
            // to set the corresponding bit in the result.
            // (31 - i) gives the bit's reversed position.
            result |= (x & 1) << (31 - i);

            // Shift `n` to the right by 1 bit to process the next bit in the next iteration.
            x >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_bits() {
        assert_eq!(964176192, Solution::reverse_bits(43261596));
    }
}
