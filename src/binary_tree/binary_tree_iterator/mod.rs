use crate::binary_tree::TreeNode;
use std::{cell::RefCell, rc::Rc};

// https://leetcode.com/problems/binary-search-tree-iterator/description/
struct BSTIterator {
    vals: Vec<i32>,
    idx: i32,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut vals = Vec::new();
        Self::inorder_traverse(&root, &mut vals);
        BSTIterator { vals, idx: -1 }
    }

    fn inorder_traverse(root: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        match root {
            Some(node) => {
                let node = node.borrow();
                Self::inorder_traverse(&node.left, vals);
                vals.push(node.val);
                Self::inorder_traverse(&node.right, vals);
            }
            None => (),
        }
    }

    fn next(&mut self) -> i32 {
        self.idx += 1;
        self.vals[self.idx as usize]
    }

    fn has_next(&self) -> bool {
        self.idx as usize + 1 < self.vals.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bst_iterator() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        let mut bst_iter = BSTIterator::new(root);
        assert_eq!(3, bst_iter.next());
        assert_eq!(7, bst_iter.next());

        assert!(bst_iter.has_next());
        assert_eq!(9, bst_iter.next());

        assert!(bst_iter.has_next());
        assert_eq!(15, bst_iter.next());

        assert!(bst_iter.has_next());
        assert_eq!(20, bst_iter.next());

        assert!(!bst_iter.has_next());
    }
}
