// https://leetcode.com/problems/same-tree/description/
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::is_same_tree_rec(&p, &q)
    }

    fn is_same_tree_rec(
        p: &Option<Rc<RefCell<TreeNode>>>,
        q: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(pn), Some(qn)) => {
                let (pn, qn) = (pn.borrow(), qn.borrow());
                pn.val == qn.val
                    && Self::is_same_tree_rec(&pn.left, &qn.left)
                    && Self::is_same_tree_rec(&pn.right, &qn.right)
            }
            (None, None) => true,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_same_tree() {
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let q = p.clone();
        assert!(Solution::is_same_tree(p, q));
    }
}
