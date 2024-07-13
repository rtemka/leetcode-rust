use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// https://leetcode.com/problems/unique-binary-search-trees-ii/description/
struct Solution {}

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut m = HashMap::with_capacity(n as usize);
        Self::gen_trees_memo(&mut m, n)
    }

    pub fn num_trees(n: i32) -> i32 {
        let mut m = HashMap::with_capacity(n as usize);
        Self::num_trees_memo2(&mut m, n)
    }

    pub fn gen_trees_memo(
        m: &mut HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>>,
        n: i32,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if let Some(v) = m.get(&n) {
            return v.clone();
        }
        let res = if n == 1 {
            vec![Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })))]
        } else {
            vec![Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })))]
        };
        m.insert(n, res.clone());
        res
    }

    pub fn num_trees_memo2(m: &mut HashMap<i32, i32>, n: i32) -> i32 {
        if let Some(v) = m.get(&n) {
            return *v;
        }
        let res = if n == 0 {
            1
        } else if n < 3 {
            n
        } else {
            let mut r = 0;
            for i in 0..n {
                r += Self::num_trees_memo2(m, i) * Self::num_trees_memo2(m, n - 1 - i);
            }
            r
        };
        m.insert(n, res);
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn num_trees() {
        assert_eq!(5, Solution::num_trees(3));
        assert_eq!(1, Solution::num_trees(1));
        assert_eq!(14, Solution::num_trees(4));
    }
}
