// https://leetcode.com/problems/binary-tree-postorder-traversal/description
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values = Vec::new();
        Self::postorder_traversal_rec(&root, &mut values);
        values
    }

    fn postorder_traversal_rec(root: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::postorder_traversal_rec(&node.left, values);
            Self::postorder_traversal_rec(&node.right, values);
            values.push(node.val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postorder_traversal() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        assert_eq!(vec![3, 2, 1], Solution::postorder_traversal(root));
    }
}
