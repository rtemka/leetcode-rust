// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/description
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut levels = Vec::new();
        Self::max_level_sum_rec(&root, &mut levels, 0);
        let mut max: (usize, i32) = (0, levels[0]);
        for (i, &sum) in levels.iter().enumerate() {
            if sum > max.1 {
                (max.0, max.1) = (i, sum);
            }
        }
        max.0 as i32 + 1
    }

    fn max_level_sum_rec(
        root: &Option<Rc<RefCell<TreeNode>>>,
        levels: &mut Vec<i32>,
        level: usize,
    ) {
        if let Some(node) = root {
            let level = level + 1;
            let node = node.borrow();
            if levels.len() < level {
                levels.push(node.val);
            } else {
                levels[level - 1] += node.val;
            }
            Self::max_level_sum_rec(&node.left, levels, level);
            Self::max_level_sum_rec(&node.right, levels, level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_level_sum() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -8,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(2, Solution::max_level_sum(root));
    }
}
