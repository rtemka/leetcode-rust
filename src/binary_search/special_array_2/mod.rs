use std::collections::HashSet;

// https://leetcode.com/problems/special-array-ii/description
struct Solution;

impl Solution {
    // FAIL: TLE
    pub fn is_array_special_tle(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prev = nums[0] % 2;
        let set: HashSet<usize> = nums
            .into_iter()
            .enumerate()
            .skip(1)
            .filter_map(|(i, x)| {
                let t = x % 2 == prev;
                prev = x % 2;
                t.then_some(i - 1)
            })
            .collect();
        let mut res = Vec::new();
        'outer: for q in queries {
            for i in q[0]..q[1] {
                if set.contains(&(i as usize)) {
                    res.push(false);
                    continue 'outer;
                }
            }
            res.push(true);
        }
        res
    }

    // Using prefix sum array.
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // Build the prefix array
        let prefix: Vec<usize> = {
            let prev = if nums[0] % 2 == 0 { 1 } else { 0 };
            nums.into_iter()
                .scan((prev, 0), |(prev, count), x| {
                    *count += (x % 2 == *prev) as usize;
                    *prev = x % 2;
                    Some(*count)
                })
                .collect()
        };
        // Process each query
        queries
            .into_iter()
            .map(|q| {
                let (lo, hi) = (q[0] as usize, q[1] as usize);
                // Calculate the number of special pairs in the range
                let special_count = prefix[hi] - {
                    if lo > 0 {
                        prefix[lo]
                    } else {
                        0
                    }
                };
                special_count == 0
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_array_special_2() {
        assert_eq!(
            vec![false, true],
            Solution::is_array_special(vec![4, 3, 1, 6], vec![vec![0, 2], vec![2, 3]])
        );
    }
}
