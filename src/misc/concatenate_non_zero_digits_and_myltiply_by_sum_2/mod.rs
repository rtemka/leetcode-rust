// #3756
// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-ii
// See also: #3754

struct Solution;

impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let divisor = 10_i64.pow(9) + 7;
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concatenate_non_zero_digits_and_multiply_by_sum_ii() {
        assert_eq!(
            vec![12340, 4, 9],
            Solution::sum_and_multiply(
                "10203004".to_string(),
                vec![vec![0, 7], vec![1, 3], vec![4, 6]]
            )
        );
    }
}
