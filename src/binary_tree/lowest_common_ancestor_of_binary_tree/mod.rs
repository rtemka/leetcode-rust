// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/description
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::lowest_common_ancestor_rec(&root, p?.borrow().val, q?.borrow().val, &mut 0)
    }

    fn lowest_common_ancestor_rec(
        root: &Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
        points: &mut i8,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let node = node.borrow();
                let node_points = (node.val == p) as i8 + (node.val == q) as i8;
                let upstream_points = *points;
                let l = Self::lowest_common_ancestor_rec(&node.left, p, q, points);
                if l.is_some() {
                    return l;
                }
                if *points + node_points - upstream_points == 2 {
                    return root.clone();
                }
                let r = Self::lowest_common_ancestor_rec(&node.right, p, q, points);
                if r.is_some() {
                    return r;
                }
                if *points + node_points - upstream_points == 2 {
                    return root.clone();
                }
                *points += node_points;
                None
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_common_ancestor() {
        let q = Rc::new(RefCell::new(TreeNode::new(4)));
        let q2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: None,
                right: None,
            }))),
        })));
        let ancestor = Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(q.clone()),
            }))),
        }));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(ancestor.clone()),
            right: q2.clone(),
        })));

        assert_eq!(
            ancestor.borrow().val,
            Solution::lowest_common_ancestor(root.clone(), Some(ancestor.clone()), Some(q.clone()))
                .unwrap()
                .borrow()
                .val
        );
        assert_eq!(
            3,
            Solution::lowest_common_ancestor(root, Some(ancestor.clone()), q2)
                .unwrap()
                .borrow()
                .val
        );
    }
}
