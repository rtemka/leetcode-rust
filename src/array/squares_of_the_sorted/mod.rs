struct Solution {}

// https://leetcode.com/problems/squares-of-a-sorted-array/
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut out: Vec<i32> = vec![0; nums.len()];
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        let mut i = hi;
        while lo != hi {
            let x = nums[lo] * nums[lo];
            let y = nums[hi] * nums[hi];
            if x > y {
                out[i] = x;
                lo += 1;
            } else {
                out[i] = y;
                hi -= 1;
            }
            i -= 1;
        }
        out[i] = nums[hi] * nums[hi];
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sorted_squares() {
        assert_eq!(
            vec![0, 1, 9, 16, 100],
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10])
        );
        assert_eq!(
            vec![4, 9, 9, 49, 121],
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11])
        );
        assert_eq!(
            vec![1, 4, 9, 25],
            Solution::sorted_squares(vec![-5, -3, -2, -1])
        );
        assert_eq!(
            vec![0, 1, 1, 4, 9, 16, 25, 36, 49, 64, 81, 121, 225],
            Solution::sorted_squares(vec![-11, -8, -5, -3, -2, -1, 0, 1, 4, 6, 7, 9, 15])
        );
        assert_eq!(vec![1], Solution::sorted_squares(vec![1]));
    }
}
