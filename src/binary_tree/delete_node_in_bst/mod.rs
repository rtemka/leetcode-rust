// https://leetcode.com/problems/delete-node-in-a-bst/description
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
impl Solution {
    pub fn delete_node(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if Self::delete_node_helper(&mut root, key).is_some() {
            None
        } else {
            root
        }
    }

    pub fn delete_node_helper(root: &mut Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<()> {
        match root {
            Some(node) if node.borrow_mut().val != key => {
                let mut node = node.borrow_mut();
                if node.val > key {
                    if Self::delete_node_helper(&mut node.left, key).is_some() {
                        node.left = None;
                        None
                    } else {
                        None
                    }
                } else if Self::delete_node_helper(&mut node.right, key).is_some() {
                    node.right = None;
                    None
                } else {
                    None
                }
            }
            Some(node) => {
                let (n, swap) = {
                    let mut n = node.borrow_mut();
                    if n.left.is_some() && n.right.is_none() {
                        (n.left.take(), true)
                    } else if n.right.is_some() && n.left.is_none() {
                        (n.right.take(), true)
                    } else {
                        (Self::take_successor(&mut n.right), false)
                    }
                };
                if let Some(n) = n {
                    if swap {
                        node.swap(n.deref());
                    } else {
                        let mut node = node.borrow_mut();
                        let mut n = n.borrow_mut();
                        node.val = n.val;
                        if n.right.is_some() {
                            Self::attach_to_right(&mut n.right, node.right.take());
                            node.right = n.right.take();
                        }
                    }
                    None
                } else {
                    Some(())
                }
            }
            None => None,
        }
    }

    fn take_successor(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = match root {
            Some(node) => {
                let mut node = node.borrow_mut();
                if node.left.is_some() {
                    Self::take_successor(&mut node.left)
                } else {
                    None
                }
            }
            None => None,
        };
        if node.is_some() {
            node
        } else {
            root.take()
        }
    }

    fn attach_to_right(
        root: &mut Option<Rc<RefCell<TreeNode>>>,
        node_to_attach: Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            if node.right.is_none() {
                node.right = node_to_attach;
            } else {
                Self::attach_to_right(&mut node.right, node_to_attach);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bst_delete_node() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let ans = Solution::delete_node(root, 3);
        assert!(ans.is_some());
        let _expected = [5, 4, 6, 2, 7];
    }
}
