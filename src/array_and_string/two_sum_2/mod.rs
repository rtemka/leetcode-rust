// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
struct Solution {}

impl Solution {
    // O(n2) soluiton
    pub fn two_sum_naive(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            for j in i + 1..numbers.len() {
                if numbers[i] + numbers[j] == target {
                    return vec![i as i32 + 1, j as i32 + 1];
                }
            }
        }
        unreachable!("do'h");
    }

    // O(n) solution
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut lo, mut hi) = (0, numbers.len() - 1);
        while lo < hi {
            let cur_sum = numbers[lo] + numbers[hi];
            if cur_sum == target {
                return vec![lo as i32 + 1, hi as i32 + 1];
            }
            if cur_sum < target {
                lo += 1;
            } else {
                hi -= 1;
            }
        }
        unreachable!("do'h");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum2() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 3], Solution::two_sum(vec![2, 3, 4], 6));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![-1, 0], -1));
        assert_eq!(
            vec![3, 7],
            Solution::two_sum(vec![4, 77, 100, 150, 250, 350, 650], 750)
        );
    }
}
