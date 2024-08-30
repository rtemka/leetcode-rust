// https://leetcode.com/problems/validate-binary-search-tree/description/
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_rec(&root, i64::MIN, i64::MAX)
    }

    fn is_valid_bst_rec(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                (node.val as i64) > min
                    && (node.val as i64) < max
                    && Self::is_valid_bst_rec(&node.left, min, node.val as i64)
                    && Self::is_valid_bst_rec(&node.right, node.val as i64, max)
            }
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_bst() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        assert!(Solution::is_valid_bst(root));
    }
}
