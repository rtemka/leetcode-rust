use std::collections::HashSet;

// https://leetcode.com/problems/contains-duplicate/description/
struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for i in nums {
            if !set.insert(i) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_duplicates() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 4, 1, 5, 6]));
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }
}
