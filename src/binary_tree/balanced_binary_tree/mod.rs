// #110
// https://leetcode.com/problems/balanced-binary-tree/description/
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::height_imbalance(&root).is_ok()
    }

    fn height_imbalance(root: &Option<Rc<RefCell<TreeNode>>>) -> Result<usize, ()> {
        // Base case: if the node is null, it's height is 0.
        if root.is_none() {
            return Ok(0);
        }
        // Recursively get the height of the left and right subtrees.
        // If either subtree is imbalanced, propagate Err(()) up the tree.
        let node = root.as_ref().ok_or(())?.borrow();
        let left = Self::height_imbalance(&node.left)?;
        let right = Self::height_imbalance(&node.right)?;
        // If the current node's subtree is imbalanced
        // (height difference > 1), return Err(()).
        if usize::abs_diff(left, right) > 1 {
            Err(())
        } else {
            // Return the height of the current subtree.
            Ok(1 + usize::max(left, right))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn balanced_binary_tree() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
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
        })));
        assert!(Solution::is_balanced(root))
    }
}
