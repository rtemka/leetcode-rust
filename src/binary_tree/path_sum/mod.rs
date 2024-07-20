// https://leetcode.com/problems/path-sum/
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::has_path_sum_rec(&root, target_sum, 0)
    }

    pub fn has_path_sum_rec(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        current_sum: i32,
    ) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => target_sum == current_sum + node.val,
                    _ => {
                        Self::has_path_sum_rec(&node.left, target_sum, current_sum + node.val)
                            || Self::has_path_sum_rec(
                                &node.right,
                                target_sum,
                                current_sum + node.val,
                            )
                    }
                }
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_path_sum() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 13,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));
        assert!(Solution::has_path_sum(tree.clone(), 26));
        assert!(Solution::has_path_sum(tree.clone(), 22));
        assert!(Solution::has_path_sum(tree.clone(), 18));
        assert!(Solution::has_path_sum(tree.clone(), 27));
        assert!(!Solution::has_path_sum(tree.clone(), 100));
    }
}
