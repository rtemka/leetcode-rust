// #2574
// https://leetcode.com/problems/left-and-right-sum-differences

struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let left_sum: Vec<i32> = nums
            .iter()
            .scan(0, |sum, &n| {
                *sum += n;
                Some(*sum)
            })
            .collect();

        let right_sum: Vec<i32> = nums
            .iter()
            .rev()
            .scan(0, |sum, &n| {
                *sum += n;
                Some(*sum)
            })
            .collect();

        left_sum
            .into_iter()
            .zip(right_sum.into_iter().rev())
            .map(|(l, r)| i32::abs_diff(l, r) as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left_and_right_sum_differences() {
        assert_eq!(
            vec![26, 15, 1, 18, 30],
            Solution::left_right_difference(vec![5, 6, 8, 11, 1])
        );

        assert_eq!(
            vec![15, 1, 11, 22],
            Solution::left_right_difference(vec![10, 4, 8, 3])
        );

        assert_eq!(vec![0], Solution::left_right_difference(vec![1]));
    }
}
