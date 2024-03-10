// https://leetcode.com/problems/height-checker/description/
struct Solution {}

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort();
        let mut count = 0;
        for i in 0..heights.len() {
            if heights[i] != expected[i] {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn height_checker() {
        assert_eq!(3, Solution::height_checker(vec![1, 1, 4, 2, 1, 3]));
        assert_eq!(5, Solution::height_checker(vec![5, 1, 2, 3, 4]));
        assert_eq!(0, Solution::height_checker(vec![1, 2, 3, 4, 5]));
    }
}
