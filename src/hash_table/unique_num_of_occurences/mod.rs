use std::collections::{HashMap, HashSet};

// https://leetcode.com/problems/unique-number-of-occurrences/description
struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in arr {
            map.entry(i).and_modify(|count| *count += 1).or_insert(1);
        }
        let mut set: HashSet<usize> = HashSet::new();
        for &v in map.values() {
            if !set.insert(v) {
                return false;
            }
        }
        println!("{:?}{:?}", map, set);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_occurrences() {
        assert!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
        assert!(!Solution::unique_occurrences(vec![1, 2]));
        assert!(Solution::unique_occurrences(vec![
            -3, 0, 1, -3, 1, 1, 1, -3, 10, 0
        ]));
    }
}
