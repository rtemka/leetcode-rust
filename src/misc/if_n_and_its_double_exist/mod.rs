use std::collections::HashMap;
struct Solution {}

// https://leetcode.com/problems/check-if-n-and-its-double-exist/description/
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(arr.len());
        for (i, n) in arr.iter().enumerate() {
            map.insert(*n, i);
        }
        for (i, n) in arr.iter().enumerate() {
            match map.get(&(n * 2)) {
                Some(v) if *v != i => return true,
                _ => (),
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_exist() {
        assert!(Solution::check_if_exist(vec![10, 2, 5, 3]));
        assert!(!Solution::check_if_exist(vec![3, 1, 7, 11]));
        assert!(!Solution::check_if_exist(vec![]));
        assert!(!Solution::check_if_exist(vec![1]));
    }
}
