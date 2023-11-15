struct Solution {}

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        for banks in accounts.iter() {
            let mut cur = 0;
            for amount in banks.iter() {
                cur += amount;
            }
            max = if cur > max { cur } else { max }
        }
        max
    }

    pub fn maximum_wealth_fold(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().fold(0, |acc, v| {
            let curr = v.iter().sum();
            if curr > acc {
                curr
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    /// cargo t maximum_wealth
    #[test]
    fn maximum_wealth() {
        assert_eq!(
            6,
            Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]])
        );

        assert_eq!(
            10,
            Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]])
        );
    }

    #[test]
    fn maximum_wealth_fold() {
        assert_eq!(
            6,
            Solution::maximum_wealth_fold(vec![vec![1, 2, 3], vec![3, 2, 1]])
        );
        assert_eq!(
            10,
            Solution::maximum_wealth_fold(vec![vec![1, 5], vec![7, 3], vec![3, 5]])
        );
    }
}
