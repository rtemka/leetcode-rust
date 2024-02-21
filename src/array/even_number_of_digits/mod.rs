struct Solution {}

// https://leetcode.com/problems/find-numbers-with-even-number-of-digits/
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut evens = 0;
        for n in nums.iter() {
            let mut num = *n;
            let mut count = 0;
            while num != 0 {
                num /= 10;
                count += 1;
            }
            evens += if count % 2 == 0 { 1 } else { 0 }
        }
        evens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_numbers() {
        assert_eq!(2, Solution::find_numbers(vec![12, 345, 2, 6, 7896]));
        assert_eq!(1, Solution::find_numbers(vec![555, 901, 482, 1771]));
    }
}
