// https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct/description/
struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for n in &nums {
            map.entry(*n).and_modify(|count| *count += 1).or_insert(1);
        }
        let mut map: HashMap<i32, usize> = map.into_iter().filter(|(_, v)| *v > 1).collect();
        let mut count = 0;
        let mut s = &nums[..];
        while !map.is_empty() && s.len() > 0 {
            let mut i = 0;
            while i < s.len() && i < 3 {
                if let Some(v) = map.get_mut(&s[i]) {
                    *v -= 1;
                    if *v < 2 {
                        map.remove(&s[i]);
                    }
                }
                i += 1;
            }
            s = &s[i..];
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_num_of_operations_to_make_elements_of_array_distinct() {
        assert_eq!(
            2,
            Solution::minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7])
        );

        assert_eq!(2, Solution::minimum_operations(vec![4, 5, 6, 4, 4]));

        assert_eq!(0, Solution::minimum_operations(vec![6, 7, 8, 9]));
    }
}
