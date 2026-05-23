// https://leetcode.com/problems/non-overlapping-intervals/description
struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|va, vb| va[1].cmp(&vb[1]));
        let mut range = intervals[0][0]..intervals[0][1];
        let mut res = 0;
        for i in 1..intervals.len() {
            let r = intervals[i][0]..intervals[i][1];
            if r.start < range.end {
                res += 1;
            } else {
                range = r;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_overlapping_intervals() {
        assert_eq!(
            1,
            Solution::erase_overlap_intervals(vec![
                vec![1, 2,],
                vec![2, 3],
                vec![3, 4],
                vec![1, 3]
            ])
        );
        assert_eq!(
            2,
            Solution::erase_overlap_intervals(vec![vec![1, 2,], vec![1, 2,], vec![1, 2,],])
        );
        assert_eq!(
            0,
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]])
        );
    }
}
