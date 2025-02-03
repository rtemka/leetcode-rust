// https://leetcode.com/problems/single-number-ii/description/
struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // for each bit position you count all numbers
        // which have that bit pos turned 1.
        // Now, if that count is not a multiple of 3, then
        // that bit must also be present in the non-repeating number.
        let mut bits = [0; 32];
        for num in nums {
            let mut num = num;
            for i in 0..32 {
                bits[i] += (num & 1 == 1) as i32;
                num >>= 1;
            }
        }
        // println!("{:?}", bits);
        let mut single = 0;
        for count in bits.into_iter().rev() {
            if count % 3 != 0 {
                single = (single << 1) | 1;
            } else {
                single <<= 1;
            }
        }
        // println!("{:#034b}", single);
        single
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number_2() {
        assert_eq!(3, Solution::single_number(vec![2, 4, 2, 3, 2, 4, 4]));
        assert_eq!(3, Solution::single_number(vec![2, 2, 3, 2,]));
        assert_eq!(
            3,
            Solution::single_number(vec![i32::MAX, i32::MAX, 3, i32::MAX])
        );
        assert_eq!(
            3,
            Solution::single_number(vec![i32::MIN, i32::MIN, 3, i32::MIN])
        );
        assert_eq!(i32::MIN, Solution::single_number(vec![3, 3, 3, i32::MIN]));
        assert_eq!(3, Solution::single_number(vec![-100, -100, 3, -100]));
    }
}
