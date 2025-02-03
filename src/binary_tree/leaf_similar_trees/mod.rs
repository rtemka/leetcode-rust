use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::collect_leafs(root1).eq(&Self::collect_leafs(root2))
    }

    fn collect_leafs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        Self::collect_leafs_rec(&root, &mut v);
        v
    }

    fn collect_leafs_rec(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            match (&node.left, &node.right) {
                (None, None) => v.push(node.val),
                (_, _) => {
                    Self::collect_leafs_rec(&node.left, v);
                    Self::collect_leafs_rec(&node.right, v);
                }
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leaf_similar() {
        let t1 = TreeNode {
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
        };
        let t2 = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        };
        assert!(!Solution::leaf_similar(
            Some(Rc::new(RefCell::new(t1))),
            Some(Rc::new(RefCell::new(t2)))
        ));
    }
}
