// #662
// https://leetcode.com/problems/maximum-width-of-binary-tree/description/
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let mut max_width = 0;
            let mut queue = VecDeque::new();
            queue.push_back((node.clone(), 0));
            while !queue.is_empty() {
                let level_size = queue.len();
                let leftmost_index = match queue.front() {
                    Some((_, index)) => *index,
                    None => 0,
                };
                let mut rightmost_index = leftmost_index;
                for _ in 0..level_size {
                    if let Some((node, index)) = queue.pop_front() {
                        let node = node.borrow();
                        if let Some(left_node) = &node.left {
                            queue.push_back((left_node.clone(), 2 * index + 1));
                        }
                        if let Some(right_node) = &node.right {
                            queue.push_back((right_node.clone(), 2 * index + 2));
                        }
                        rightmost_index = index;
                    }
                }
                max_width = max_width.max(rightmost_index - leftmost_index + 1);
            }
            max_width
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_width_of_binary_tree() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    ..Default::default()
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    ..Default::default()
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    ..Default::default()
                }))),
            }))),
        })));
        assert_eq!(4, Solution::width_of_binary_tree(root));
    }
}
