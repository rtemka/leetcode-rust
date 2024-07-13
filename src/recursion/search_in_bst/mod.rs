struct Solution {}

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/search-in-a-binary-search-tree/description/
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(rc) => {
                let node = rc.borrow();
                if val == node.val {
                    Some(rc.to_owned())
                } else if val < node.val {
                    Self::search_bst(node.left.clone(), val)
                } else {
                    Self::search_bst(node.right.clone(), val)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_bst() {
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
        let result = Solution::search_bst(Some(bst.clone()), 2);
        assert_eq!(2, result.unwrap().borrow().val);

        let result2 = Solution::search_bst(Some(bst.clone()), 5);
        assert_eq!(None, result2);

        let result3 = Solution::search_bst(None, 5);
        assert_eq!(None, result3);
    }
}
