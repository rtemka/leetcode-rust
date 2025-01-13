// https://leetcode.com/problems/single-number-iii/description/
struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // One important point is that by XORing all the numbers, we actually get the XOR of the two target numbers
        // (because XORing two duplicate numbers always results in 0). Consider the XOR result of the two target numbers;
        // if some bit of the XOR result is 1, it means that the two target numbers differ at that location.

        // Let’s say the at the ith bit, the two desired numbers differ from each other. which means one number has bit i equaling: 0, the other number has bit i equaling 1.

        // Thus, all the numbers can be partitioned into two groups according to their bits at location i.
        // the first group consists of all numbers whose bits at i is 0.
        // the second group consists of all numbers whose bits at i is 1.

        // Notice that, if a duplicate number has bit i as 0, then, two copies of it will belong to the first group.
        // Similarly, if a duplicate number has bit i as 1, then, two copies of it will belong to the second group.

        // by XoRing all numbers in the first group, we can get the first number.
        // by XoRing all numbers in the second group, we can get the second number.

        let xor = nums.iter().fold(0, |acc, num| acc ^ num);
        let shift = (0..32)
            // find the first set bit in XORed result
            .skip_while(|i| (xor >> i) & 1 == 0)
            .next()
            .unwrap_or_default();

        let first = nums
            .iter()
            .filter(|&num| (num >> shift) & 1 == 1)
            .fold(0, |acc, &num| acc ^ num);

        let second = nums
            .iter()
            .filter(|&num| (num >> shift) & 1 == 0)
            .fold(0, |acc, &num| acc ^ num);

        vec![first, second]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_number_3() {
        let mut got = Solution::single_number(vec![1, 2, 1, 3, 2, 5]);
        got.sort_unstable();
        assert_eq!(vec![3, 5], got);

        let mut got = Solution::single_number(vec![-1, 2147483647]);
        got.sort_unstable();
        assert_eq!(vec![-1, 2147483647], got,);

        let mut got = Solution::single_number(vec![6, 6, 1, 2, 1, 3, 2, 4, 5, 4]);
        got.sort_unstable();
        assert_eq!(vec![3, 5], got,);

        let mut got = Solution::single_number(vec![-1, 0]);
        got.sort_unstable();
        assert_eq!(vec![-1, 0], got);
    }
}
