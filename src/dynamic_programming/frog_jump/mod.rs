// #403
// https://leetcode.com/problems/frog-jump/description/
struct Solution;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frog_jump() {
        assert!(Solution::can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]));
    }
}
