struct Solution {}

// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1;
        let mut hi = nums.len();
        while i < hi {
            if nums[i - 1] == nums[i] {
                for i in i..hi {
                    nums[i - 1] = nums[i];
                }
                hi -= 1;
                continue;
            }
            i += 1;
        }
        nums.truncate(hi);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplicates() {
        let mut arr = vec![1, 1, 2];
        assert_eq!(2, Solution::remove_duplicates(&mut arr));
        assert_eq!(vec![1, 2], arr);

        let mut arr = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, Solution::remove_duplicates(&mut arr));
        assert_eq!(vec![0, 1, 2, 3, 4], arr);
    }
}
