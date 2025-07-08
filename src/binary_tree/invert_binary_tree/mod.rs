// #226
// https://leetcode.com/problems/invert-binary-tree/description/
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert_tree_rec(&mut root);
        root
    }

    fn invert_tree_rec(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // If the node is null, there's nothing to invert.
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            // Swap the left and right subtrees of the current node.
            (node.left, node.right) = (node.right.take(), node.left.take());
            // Recursively invert the left and right subtrees.
            Self::invert_tree_rec(&mut node.left);
            Self::invert_tree_rec(&mut node.right);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invert_binary_tree() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    ..Default::default()
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    ..Default::default()
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    ..Default::default()
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    ..Default::default()
                }))),
            }))),
        })));
        let inverted = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    ..Default::default()
                }))),
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    ..Default::default()
                }))),
            }))),
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    ..Default::default()
                }))),
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    ..Default::default()
                }))),
            }))),
        })));
        assert_eq!(Solution::invert_tree(root), inverted);
    }
}
