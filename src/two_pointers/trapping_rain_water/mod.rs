// https://leetcode.com/problems/trapping-rain-water/description/
struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left: Vec<i32> = vec![0; height.len()];
        let mut right: Vec<i32> = vec![0; height.len()];

        left[0] = height[0];
        right[height.len() - 1] = height[height.len() - 1];

        for i in 1..height.len() {
            left[i] = i32::max(left[i - 1], height[i]);
        }

        for i in (0..height.len() - 1).rev() {
            right[i] = i32::max(right[i + 1], height[i]);
        }

        let mut trapped = 0;
        for i in 0..height.len() {
            trapped += i32::min(left[i], right[i]) - height[i];
        }
        trapped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trapping_rain_water() {
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }
}
