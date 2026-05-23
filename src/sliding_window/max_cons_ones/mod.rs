struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut running_max = 0;
        for i in nums.iter() {
            if *i == 1 {
                running_max += 1;
                continue;
            }
            if running_max > max {
                max = running_max;
            }
            running_max = 0;
        }
        if running_max > max {
            running_max
        } else {
            max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_consecutive_ones() {
        assert_eq!(
            3,
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
        );
        assert_eq!(
            2,
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 0, 1, 1])
        );
        assert_eq!(
            2,
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1])
        )
    }
}
