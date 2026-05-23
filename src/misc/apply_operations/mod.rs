// https://leetcode.com/problems/apply-operations-to-an-array/description
struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                nums[i - 1] *= 2;
                nums[i] = 0;
            }
        }
        'outer: for i in 0..nums.len() {
            if nums[i] == 0 {
                for k in i + 1..nums.len() {
                    if nums[k] != 0 {
                        nums.swap(i, k);
                        continue 'outer;
                    }
                }
                break;
            }
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_operations() {
        assert_eq!(
            vec![2, 4, 8, 0, 0, 0, 0],
            Solution::apply_operations(vec![1, 1, 2, 2, 0, 4, 4])
        );
        assert_eq!(
            vec![1, 4, 2, 0, 0, 0],
            Solution::apply_operations(vec![1, 2, 2, 1, 1, 0])
        );
        assert_eq!(vec![1, 0], Solution::apply_operations(vec![0, 1]));
    }
}
