// https://leetcode.com/problems/first-missing-positive/description/
struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        // If an integer is missing it must be in the range [1..n],
        // if an integer is not missing then the answer is n+1
        //
        // ignore all numbers <= 0 and > n since they are outside the range of possible answers
        // replacing them with the value n+1.
        let n = nums.len() as i32;
        let mut i = 0;
        while i < nums.len() {
            if nums[i] <= 0 || nums[i] > n {
                nums[i] = n + 1;
            }
            i += 1;
        }
        // println!("{:?}", nums);
        // for all other integers <n+1, mark their bucket (cell) to indicate the integer exists.
        i = 0;
        while i < nums.len() {
            let num = nums[i];
            if num == n + 1 || num == -1 || nums[num as usize - 1] == -1 {
                i += 1;
            } else {
                nums[i] = -1;
                nums.swap(i, num as usize - 1);
            }
        }
        // println!("{:?}", nums);
        // Find the first cell not marked, that is the first missing integer
        i = 0;
        while i < nums.len() {
            if nums[i] != -1 {
                return i as i32 + 1;
            }
            i += 1;
        }
        // If you did not find an unmarked cell, there was no missing integer, so return n+1
        n + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_missing_positive() {
        assert_eq!(2, Solution::first_missing_positive(vec![1]));
        assert_eq!(2, Solution::first_missing_positive(vec![1, 1]));
        assert_eq!(1, Solution::first_missing_positive(vec![2, 2]));
        assert_eq!(2, Solution::first_missing_positive(vec![1, 3, 5, 0]));

        assert_eq!(
            6,
            Solution::first_missing_positive(vec![
                -3, 9, 16, 4, 5, 16, -4, 9, 26, 2, 1, 19, -1, 25, 7, 22, 2, -7, 14, 2, 5, -6, 1,
                17, 3, 24, -4, 17, 15
            ])
        );
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, -1, 1]));
        assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 2]));
    }
}
