struct Solution {}

// https://leetcode.com/problems/remove-element/description/
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut lo = 0;
        let mut hi: i32 = nums.len() as i32 - 1;
        while lo <= hi {
            if nums[lo as usize] == val {
                nums[lo as usize] = nums[hi as usize];
                nums.truncate(hi as usize);
                hi -= 1;
            } else {
                lo += 1;
            }
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_element() {
        let mut v = vec![3, 2, 2, 3];
        let mut got = Solution::remove_element(&mut v, 3);
        assert_eq!(vec![2, 2], v);
        assert_eq!(2, got);

        v = vec![0, 1, 2, 2, 3, 0, 4, 2];
        got = Solution::remove_element(&mut v, 2);
        assert_eq!(vec![0, 1, 4, 0, 3], v);
        assert_eq!(5, got);

        v = vec![];
        got = Solution::remove_element(&mut v, 0);
        assert_eq!(Vec::<i32>::new(), v);
        assert_eq!(0, got);

        v = vec![1];
        got = Solution::remove_element(&mut v, 1);
        assert_eq!(Vec::<i32>::new(), v);
        assert_eq!(0, got);

        v = vec![3, 3];
        got = Solution::remove_element(&mut v, 3);
        assert_eq!(Vec::<i32>::new(), v);
        assert_eq!(0, got);
    }
}
