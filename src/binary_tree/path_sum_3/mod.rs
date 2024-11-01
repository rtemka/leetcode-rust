// https://leetcode.com/problems/path-sum-iii/description
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut path_sum = 0;
        let mut path = Vec::with_capacity(10); // constraint: [0, 1000]
        Self::path_sum_rec(&root, &mut path, &mut path_sum, target_sum);
        path_sum
    }

    fn path_sum_rec(
        root: &Option<Rc<RefCell<TreeNode>>>,
        path: &mut Vec<i32>,
        path_sum: &mut i32,
        target_sum: i32,
    ) {
        match root {
            Some(node) => {
                let node = node.borrow();
                path.push(node.val);
                Self::path_sum_rec(&node.left, path, path_sum, target_sum);
                Self::path_sum_rec(&node.right, path, path_sum, target_sum);
                path.pop();
            }
            None => *path_sum += Self::target_sum_count(path, target_sum),
        }
    }

    #[inline(always)]
    fn target_sum_count(path: &[i32], target_sum: i32) -> i32 {
        let (lo, mut hi) = (0, 1);
        let mut cur_sum = path[lo];
        let count = if cur_sum == target_sum { 1 } else { 0 };
        // let sign = if target_sum < 0 { -1 } else { 1 };
        while hi < path.len() {
            cur_sum += path[hi];
            cur_sum == target_sum;
            hi += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_sum3() {
        let root = None;
        assert_eq!(3, Solution::path_sum(root, 8));
    }
}
