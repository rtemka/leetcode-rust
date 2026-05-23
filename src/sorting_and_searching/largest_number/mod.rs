// https://leetcode.com/problems/largest-number/description/
struct Solution;

// use std::fmt::Write;

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_unstable_by(|&a, &b| {
            let (a, b) = (a as u32, b as u32);
            let ab = a * 10_u32.pow(b.checked_ilog10().unwrap_or(0) + 1) + b;
            let ba = b * 10_u32.pow(a.checked_ilog10().unwrap_or(0) + 1) + a;
            ba.cmp(&ab)
        });
        if nums[0] == 0 {
            return "0".to_string();
        }
        nums.iter().map(ToString::to_string).collect()
        // nums.iter().fold(String::new(), |mut s, &n| {
        //     let _ = write!(s, "{}", n).ok();
        //     s
        // })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_number() {
        assert_eq!(
            "9534330".to_owned(),
            Solution::largest_number(vec![3, 30, 34, 5, 9])
        );
        assert_eq!("0".to_owned(), Solution::largest_number(vec![0, 0, 0]));
    }
}
