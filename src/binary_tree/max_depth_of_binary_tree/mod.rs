// https://leetcode.com/problems/maximum-depth-of-binary-tree/description/
struct Solution {}

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                let left = Self::max_depth(node.left.clone());
                let right = Self::max_depth(node.right.clone());
                if left + right == 0 {
                    1
                } else if left > right {
                    left + 1
                } else {
                    right + 1
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
    fn max_depth_of_bst() {
        let bst = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
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
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }));
        assert_eq!(3, Solution::max_depth(Some(bst)));
    }
}
