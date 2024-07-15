// https://leetcode.com/problems/find-duplicate-subtrees/description/
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut map: HashMap<String, usize> = HashMap::new();
        let mut v = Vec::new();
        Self::find_dup_subtrees_rec(&root, &mut map, &mut v);
        v
    }

    fn find_dup_subtrees_rec(
        root: &Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<String, usize>,
        v: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> String {
        match root {
            Some(node) => {
                let node = node.borrow();
                // serialize the subtree into string.
                let s = format!(
                    "{}:{}:{}",
                    node.val,
                    Self::find_dup_subtrees_rec(&node.left, map, v),
                    Self::find_dup_subtrees_rec(&node.right, map, v)
                );
                // if find in map and it's count is 1, then push to result.
                map.entry(s.clone())
                    .and_modify(|count| {
                        if *count == 1 {
                            v.push(root.clone());
                            *count += 1;
                        }
                    })
                    .or_insert(1);
                s
            }
            None => "#".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_duplicate_subtrees() {
        let _root = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        }));

        // fuck that!
        // let expected = vec![vec![2, 4], vec![4]];
    }
}
