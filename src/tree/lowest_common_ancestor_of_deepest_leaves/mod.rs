// https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/description
struct Solution;

use super::TreeNode;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (lca, _) = Self::lca(&root, 0);
        lca
    }

    pub fn lca(
        root: &Option<Rc<RefCell<TreeNode>>>,
        d: usize,
    ) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        match root {
            Some(node) => {
                let node = node.borrow();
                let (left, left_depth) = Self::lca(&node.left, d + 1);
                let (right, right_depth) = Self::lca(&node.right, d + 1);

                match left_depth.cmp(&right_depth) {
                    Ordering::Greater => (left, left_depth),
                    Ordering::Less => (right, right_depth),
                    Ordering::Equal => (
                        root.clone(),
                        right_depth, // any will do
                    ),
                }
            }
            None => (None, d.saturating_sub(1)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_common_ancestor_of_deepest_leaves() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    ..Default::default()
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        ..Default::default()
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        ..Default::default()
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    ..Default::default()
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    ..Default::default()
                }))),
            }))),
        })));

        let res = Solution::lca_deepest_leaves(root);
        let expected = vec![2, 7, 4];
        let head = res.unwrap();
        let head = head.borrow();
        assert_eq!(expected[0], head.val);
    }
}
