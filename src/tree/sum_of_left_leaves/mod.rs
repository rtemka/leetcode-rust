// #404
// https://leetcode.com/problems/sum-of-left-leaves/description/

use std::{cell::RefCell, rc::Rc};

use crate::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_of_left_leaves_rec(&root, false)
    }

    pub fn sum_of_left_leaves_rec(root: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() && is_left {
                    node.val
                } else {
                    Solution::sum_of_left_leaves_rec(&node.left, true)
                        + Solution::sum_of_left_leaves_rec(&node.right, false)
                }
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_left_leaves() {
        let root = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                ..Default::default()
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    ..Default::default()
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    ..Default::default()
                }))),
            }))),
        };

        assert_eq!(
            24,
            Solution::sum_of_left_leaves(Some(Rc::new(RefCell::new(root))))
        );
    }
}
