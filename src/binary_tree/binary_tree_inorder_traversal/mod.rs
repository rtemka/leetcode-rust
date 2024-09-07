// https://leetcode.com/problems/binary-tree-inorder-traversal/description/
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::inorder_traversal_rec(&root, &mut result);
        result
    }

    fn inorder_traversal_rec(root: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::inorder_traversal_rec(&node.left, result);
            result.push(node.val);
            Self::inorder_traversal_rec(&node.right, result);
        }
    }
}
