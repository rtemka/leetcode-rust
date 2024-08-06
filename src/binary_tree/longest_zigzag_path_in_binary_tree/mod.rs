struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        Self::longest_zig_zag_rec(&root, false, 0, &mut max);
        max
    }

    fn longest_zig_zag_rec(
        root: &Option<Rc<RefCell<TreeNode>>>,
        dir_left: bool,
        cur: i32,
        max: &mut i32,
    ) {
        *max = i32::max(*max, cur);
        match root {
            Some(node) => {
                let node = node.borrow();
                if dir_left {
                    Self::longest_zig_zag_rec(&node.right, false, cur + 1, max);
                    Self::longest_zig_zag_rec(&node.left, true, 0, max);
                } else {
                    Self::longest_zig_zag_rec(&node.left, true, cur + 1, max);
                    Self::longest_zig_zag_rec(&node.right, false, 0, max);
                }
            }
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_zig_zag() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 1,
                                left: None,
                                right: None,
                            }))),
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));
        assert_eq!(3, Solution::longest_zig_zag(root));
    }
}
