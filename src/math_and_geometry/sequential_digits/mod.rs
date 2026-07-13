// #1291
// https://leetcode.com/problems/sequential-digits

struct Solution;

const NUMS: [i32; 8 * 9 / 2] = [
    12, 23, 34, 45, 56, 67, 78, 89, // 2 цифры (8 чисел)
    123, 234, 345, 456, 567, 678, 789, // 3 цифры (7 чисел)
    1234, 2345, 3456, 4567, 5678, 6789, // 4 цифры (6 чисел)
    12345, 23456, 34567, 45678, 56789, // 5 цифр (5 чисел)
    123456, 234567, 345678, 456789, // 6 цифр (4 числа)
    1234567, 2345678, 3456789, // 7 цифр (3 числа)
    12345678, 23456789,  // 8 цифр (2 числа)
    123456789, // 9 цифр (1 число)
];

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let lo = NUMS.binary_search(&low).unwrap_or_else(|x| x);
        let hi = NUMS.binary_search(&high).unwrap_or_else(|x| x);

        if hi < NUMS.len() && NUMS[hi] == high {
            NUMS[lo..=hi].into()
        } else {
            NUMS[lo..hi].into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_digits() {
        assert_eq!(vec![123, 234], Solution::sequential_digits(100, 300));
        assert_eq!(
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345],
            Solution::sequential_digits(1000, 13000)
        );
    }
}
