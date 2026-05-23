// https://leetcode.com/problems/count-symmetric-integers/description/
struct Solution;

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        (low..=high)
            .filter_map(|i| {
                let s = i.to_string();
                if s.len() % 2 == 0 {
                    Some(s)
                } else {
                    None
                }
            })
            .fold(0, |acc, num| {
                let first = &num[..num.len() / 2]
                    .bytes()
                    .fold(0, |acc, b| acc + (b - 48));
                let last = &num[num.len() / 2..]
                    .bytes()
                    .fold(0, |acc, b| acc + (b - 48));
                acc + (first == last) as i32
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_symmetric_integers() {
        assert_eq!(9, Solution::count_symmetric_integers(1, 100));
        assert_eq!(4, Solution::count_symmetric_integers(1200, 1230));
        assert_eq!(0, Solution::count_symmetric_integers(1, 9));
        assert_eq!(1, Solution::count_symmetric_integers(22, 22));
        assert_eq!(1, Solution::count_symmetric_integers(99, 9999));
    }
}
