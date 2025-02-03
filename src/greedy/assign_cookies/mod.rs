// https://leetcode.com/problems/assign-cookies/description/
struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        if s.is_empty() || s[s.len() - 1] < g[0] {
            return 0;
        }
        let (mut i, mut j) = (0, 0);
        let mut content_children_count = 0;
        while i < g.len() && j < s.len() {
            if s[j] >= g[i] {
                content_children_count += 1;
                (i, j) = (i + 1, j + 1);
            } else {
                j += 1;
            }
        }

        content_children_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_content_children() {
        assert_eq!(
            2,
            Solution::find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8])
        );
        assert_eq!(0, Solution::find_content_children(vec![1, 2, 3], vec![]));
        assert_eq!(
            1,
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1])
        );
        assert_eq!(
            2,
            Solution::find_content_children(vec![1, 2,], vec![1, 2, 3])
        );
    }
}
