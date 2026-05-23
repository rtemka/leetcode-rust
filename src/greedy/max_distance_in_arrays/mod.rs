// https://leetcode.com/problems/maximum-distance-in-arrays/description
struct Solution;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let (mut cur_min, mut cur_max) = (arrays[0][0], arrays[0][arrays[0].len() - 1]);
        let mut distance = 0;
        for i in 1..arrays.len() {
            let cur_distance = i32::max(
                cur_max - arrays[i][0],
                arrays[i][arrays[i].len() - 1] - cur_min,
            );
            distance = distance.max(cur_distance);
            cur_max = cur_max.max(arrays[i][arrays[i].len() - 1]);
            cur_min = cur_min.min(arrays[i][0]);
        }
        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_distance() {
        assert_eq!(
            4,
            Solution::max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]])
        );
    }
}
