// https://leetcode.com/problems/increasing-triplet-subsequence/description
struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }
        let (mut min, mut maybe_mid) = (nums[0], None);
        for i in 1..nums.len() {
            println!("{min};{maybe_mid:?};{}", nums[i]);
            let n = nums[i];
            if let Some(mid) = maybe_mid {
                if min < n && mid < n {
                    return true;
                }
            }
            if n < min {
                min = n;
            }
            if n > min {
                maybe_mid = Some(n);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn increasing_triplet() {
        assert!(Solution::increasing_triplet(vec![5, 1, 5, 5, 2, 5, 4]));
        assert!(Solution::increasing_triplet(vec![1, 2, 1, 3]));
        assert!(Solution::increasing_triplet(vec![1, 5, 0, 4, 1, 3]));
        assert!(!Solution::increasing_triplet(vec![1]));
        assert!(!Solution::increasing_triplet(vec![0, 4, 2, 1, 0, -1, -3]));
        assert!(!Solution::increasing_triplet(vec![5, 1, 6]));
        assert!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
        assert!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
        assert!(!Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
        assert!(Solution::increasing_triplet(vec![20, 100, 10, 12, 5, 13]));
    }
}
