// #2177
// https://leetcode.com/problems/find-three-consecutive-integers-that-sum-to-a-given-number/description/

struct Solution;

impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        if num % 3 != 0 {
            return vec![];
        }

        let quotient = num / 3;

        vec![quotient - 1, quotient, quotient + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_three_consecutive_integers_that_sum_to_a_given_number() {
        assert_eq!(vec![10, 11, 12], Solution::sum_of_three(33));
        assert_eq!(Vec::<i64>::new(), Solution::sum_of_three(4));
        assert_eq!(Vec::<i64>::new(), Solution::sum_of_three(0));
    }
}
