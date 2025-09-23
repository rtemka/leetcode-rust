// #108
// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/description/
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::sorted_array_to_bst_rec(&nums)
    }

    fn sorted_array_to_bst_rec(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let mid = nums.len() / 2;
        let node = TreeNode {
            val: nums[mid],
            left: if mid == 0 {
                None
            } else {
                Self::sorted_array_to_bst_rec(&nums[..mid])
            },
            right: if mid == nums.len() - 1 {
                None
            } else {
                Self::sorted_array_to_bst_rec(&nums[mid + 1..])
            },
        };
        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_sorted_array_to_bst() {
        let want = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -10,
                    ..Default::default()
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    ..Default::default()
                }))),
                right: None,
            }))),
        })));
        assert_eq!(want, Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]))
    }
}
