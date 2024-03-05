struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut lo = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, lo);
                lo += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_zeroes() {
        let mut v = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![1, 3, 12, 0, 0], v);

        let mut v = vec![0];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![0], v);

        let mut v = vec![1, 1, 1, 1, 1];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![1, 1, 1, 1, 1], v);

        let mut v = vec![0, 0, 0, 0, 0];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![0, 0, 0, 0, 0], v);

        let mut v = vec![1, 0, 2, 0, 0, 7, 8, 9, 0, 0, 12, 15, 0];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![1, 2, 7, 8, 9, 12, 15, 0, 0, 0, 0, 0, 0], v);

        let mut v = vec![1, 0, 1];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![1, 1, 0], v);
    }
}
