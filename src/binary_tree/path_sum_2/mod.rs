// https://leetcode.com/problems/path-sum-ii/description/
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut v = Vec::new();
        Self::path_sum_rec(&root, target_sum, 0, &mut v, &mut Vec::new());
        v
    }

    pub fn path_sum_rec(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        current_sum: i32,
        res: &mut Vec<Vec<i32>>,
        cur: &mut Vec<i32>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            cur.push(node.val);
            match (&node.left, &node.right) {
                (None, None) => {
                    if target_sum == current_sum + node.val {
                        res.push(cur.clone());
                    }
                }
                _ => {
                    Self::path_sum_rec(
                        &node.left,
                        target_sum,
                        current_sum + node.val,
                        res,
                        cur,
                    );
                    Self::path_sum_rec(
                        &node.right,
                        target_sum,
                        current_sum + node.val,
                        res,
                        cur,
                    );
                }
            };
            cur.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_sum() {
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
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));
        assert_eq!(
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]],
            Solution::path_sum(tree, 22)
        );
    }
}
