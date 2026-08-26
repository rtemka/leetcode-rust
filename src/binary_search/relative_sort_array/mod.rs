// #1122
// https://leetcode.com/problems/relative-sort-array/description

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let order: HashMap<i32, usize> =
            arr2.into_iter().enumerate().map(|(i, v)| (v, i)).collect();

        arr1.sort_unstable_by(|a, b| match (order.get(a), order.get(b)) {
            (Some(ai), Some(bi)) => ai.cmp(bi),
            (None, None) => a.cmp(b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
        });
        arr1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_sort_array() {
        assert_eq!(
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19],
            Solution::relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            )
        );
    }
}
