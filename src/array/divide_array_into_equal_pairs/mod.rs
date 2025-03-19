use std::collections::HashMap;

// https://leetcode.com/problems/divide-array-into-equal-pairs/description
struct Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let m: HashMap<i32, usize> = {
            let mut m = HashMap::new();
            for n in nums {
                m.entry(n).and_modify(|count| *count += 1).or_insert(1);
            }
            m
        };
        m.into_iter().all(|(_, v)| v % 2 == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_array_into_equal_pairs() {
        assert!(Solution::divide_array(vec![3, 2, 3, 2, 2, 2]));
        assert!(!Solution::divide_array(vec![1, 2, 3, 4]));
        assert!(!Solution::divide_array(vec![1]));
        assert!(!Solution::divide_array(vec![1, 1, 1, 1, 1]));
    }
}
