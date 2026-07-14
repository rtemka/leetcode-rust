// #1846
// https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging

struct Solution;

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        arr[0] = 1;

        for i in 1..arr.len() {
            if arr[i - 1].abs_diff(arr[i]) > 1 {
                arr[i] = arr[i - 1] + 1;
            }
        }

        arr[arr.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn maximum_element_after_decreasing_and_rearranging() {
        assert_eq!(
            1,
            Solution::maximum_element_after_decrementing_and_rearranging(vec![10000])
        );

        assert_eq!(
            2,
            Solution::maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1])
        );

        assert_eq!(
            3,
            Solution::maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000])
        );

        assert_eq!(
            5,
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5])
        );
    }
}
