// https://leetcode.com/problems/minimum-depth-of-binary-tree/description/
struct Solution;

use super::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_depth_rec(&root, 0).unwrap_or_default()
    }

    fn min_depth_rec(root: &Option<Rc<RefCell<TreeNode>>>, cur: i32) -> Option<i32> {
        match root {
            Some(node) => {
                let node = node.borrow();
                let l = Self::min_depth_rec(&node.left, cur + 1);
                let r = Self::min_depth_rec(&node.right, cur + 1);
                match (l, r) {
                    (Some(lv), Some(rv)) => Some(lv.min(rv)),
                    (Some(lv), None) => Some(lv),
                    (None, Some(rv)) => Some(rv),
                    _ => Some(cur + 1),
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_depth_of_binary_tree() {
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

        assert_eq!(2, Solution::min_depth(root));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            ..Default::default()
        })));
        assert_eq!(1, Solution::min_depth(root));
    }
}
