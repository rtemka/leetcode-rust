// https://leetcode.com/problems/can-place-flowers/description
struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        for i in 0..flowerbed.len() {
            if n == 0 {
                return true;
            }
            if flowerbed[i] == 0
                && (i == 0 || flowerbed[i - 1] == 0)
                && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
            {
                (flowerbed[i], n) = (1, n - 1);
            }
        }
        n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_place_flowers() {
        assert!(Solution::can_place_flowers(vec![0, 0, 0, 0, 0], 3));
        assert!(Solution::can_place_flowers(vec![0, 0, 0, 0, 0], 1));
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2));
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
        assert!(Solution::can_place_flowers(vec![0], 1));
        assert!(Solution::can_place_flowers(vec![0, 0], 1));
        assert!(Solution::can_place_flowers(vec![0, 0, 0], 2));
        assert!(!Solution::can_place_flowers(vec![0, 0, 0], 3));
        assert!(Solution::can_place_flowers(
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0],
            3
        ));
    }
}
