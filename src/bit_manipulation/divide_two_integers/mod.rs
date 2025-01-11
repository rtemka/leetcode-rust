// https://leetcode.com/problems/divide-two-integers/description/
struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        // Let's take two numbers a = 96 and b = 7. When we divide a by b,
        // we are calculating how many times b is equal to a or how many b's
        // can fit inside a. In this case, we can fit 13 b's in a i.e. a / b = 13 (Note: here we calculate only the floor value)
        //
        // We know that each number can be represented as the sum of powers of 2
        // and also when we shift a number towards left by n bits it is multiplied by 2 power n.
        //
        // Thus, what we do is shift the divisor b towards the left and check if it is
        // less than or equal to the dividend a. If it is less than or equal to the dividend we
        // subtract it from the dividend and add the value of 2 power n to our answer.
        // Doing so, we get our answer as the sum of powers of 2, which will give us the required quotient.
        let result_is_positive = (dividend > 0 && divisor > 0) || (dividend < 0 && divisor < 0);

        let mut result = 0u32;
        // we will be operating on positive integers
        let mut a = dividend.unsigned_abs();
        let b = divisor.unsigned_abs();
        // calculate max bits we can shift to the left
        let max_bits = 32 - (31 - b.leading_zeros());

        // dbg!(a, b, dividend, divisor);
        // println!("a={a:#034b}\tb={b:#034b}");

        for i in (0..max_bits).rev() {
            if b << i <= a {
                a -= b << i;
                result += 1 << i; // 2 to the power of i.
            }
        }
        match result_is_positive {
            // f..kin special case for i32::MIN / -1
            true if result > i32::MAX as u32 => i32::MAX, // this is bullshit (compiler will warn us that i32::MIN/-1 would overflow, so wtf is this?
            false if result > i32::MAX as u32 => i32::MIN,
            true => result as i32,
            false => -(result as i32),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_two_integers() {
        assert_eq!(2147483647, Solution::divide(i32::MIN, -1));
        assert_eq!(
            -1_021_989_372 / -82_778_243,
            Solution::divide(-1021989372, -82778243)
        );
        assert_eq!(
            1004958205 / -2137325331,
            Solution::divide(1004958205, -2137325331)
        );
        assert_eq!(10 / 3, Solution::divide(10, 3));
        assert_eq!(-10 / -3, Solution::divide(-10, -3));
        assert_eq!(i32::MAX / i32::MAX, Solution::divide(i32::MAX, i32::MAX));
        assert_eq!(i32::MAX / 2, Solution::divide(i32::MAX, 2));
        assert_eq!(i32::MAX / 1, Solution::divide(i32::MAX, 1));
        assert_eq!(i32::MIN / i32::MIN, Solution::divide(i32::MIN, i32::MIN));
        assert_eq!(i32::MIN / 1, Solution::divide(i32::MIN, 1));
    }
}
