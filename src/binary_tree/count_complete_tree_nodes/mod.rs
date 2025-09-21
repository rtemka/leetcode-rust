// #222
// https://leetcode.com/problems/count-complete-tree-nodes/description/
struct Solution;

use crate::binary_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::count_nodes_rec(&root)
    }

    fn count_nodes_rec(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                1 + Self::count_nodes_rec(&node.left) + Self::count_nodes_rec(&node.right)
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete_tree_nodes() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    ..Default::default()
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    ..Default::default()
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    ..Default::default()
                }))),
                right: None,
            }))),
        })));
        assert_eq!(6, Solution::count_nodes(root));
    }
}
