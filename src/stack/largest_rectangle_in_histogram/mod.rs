// #84
// https://leetcode.com/problems/largest-rectangle-in-histogram/description/
// Solution: https://www.youtube.com/watch?v=zx5Sw9130L0
// Also: https://algo.monster/liteproblems/84

struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let n = heights.len();
        let mut stack: Vec<(usize, i32)> = Vec::new();

        for (i, height) in heights.into_iter().enumerate() {
            let mut start = i;

            while let Some(&(prev_idx, prev_height)) = stack.last()
                && prev_height > height
            {
                max_area = max_area.max(prev_height * (i - prev_idx) as i32);
                start = prev_idx;
                stack.pop();
            }

            stack.push((start, height));
        }

        for (i, height) in stack.into_iter() {
            max_area = max_area.max(height * (n - i) as i32);
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_rectangle_in_histogram() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 0, 2]), 2);
        assert_eq!(Solution::largest_rectangle_area(vec![4, 2]), 4);
        assert_eq!(Solution::largest_rectangle_area(vec![0]), 0);
        assert_eq!(Solution::largest_rectangle_area(vec![1]), 1);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }
}
