struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::good_nodes_rec(&root, i32::MIN)
    }

    fn good_nodes_rec(root: &Option<Rc<RefCell<TreeNode>>>, cur_max: i32) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                let this_good = (node.val >= cur_max) as i32;
                let cur_max = cur_max.max(node.val);
                this_good
                    + Self::good_nodes_rec(&node.left, cur_max)
                    + Self::good_nodes_rec(&node.right, cur_max)
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_nodes() {
        let tree = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
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
        };
        assert_eq!(3, Solution::good_nodes(Some(Rc::new(RefCell::new(tree)))));
    }
}
