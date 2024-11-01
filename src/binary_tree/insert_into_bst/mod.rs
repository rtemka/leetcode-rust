// https://leetcode.com/problems/insert-into-a-binary-search-tree/description/
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn insert_into_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_some() {
            Self::insert_into_bst_rec(&mut root, val);
        } else {
            root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        root
    }

    fn insert_into_bst_rec(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if val > node.val {
                if node.right.is_some() {
                    Self::insert_into_bst_rec(&mut node.right, val);
                } else {
                    node.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                }
            } else if node.left.is_some() {
                Self::insert_into_bst_rec(&mut node.left, val);
            } else {
                node.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_into_bst() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    ..TreeNode::default()
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    ..TreeNode::default()
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                ..TreeNode::default()
            }))),
        })));
        let ans = Solution::insert_into_bst(root, 5);
        assert_eq!(
            vec![1, 2, 3, 4, 5, 7],
            crate::binary_tree::binary_tree_inorder_traversal::Solution::inorder_traversal(ans)
        );
    }
}
