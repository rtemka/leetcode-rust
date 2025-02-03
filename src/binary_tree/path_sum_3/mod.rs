// https://leetcode.com/problems/path-sum-iii/description
struct Solution;

use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
impl Solution {
    const LEFT: char = '<';
    const RIGHT: char = '>';

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut path_sum = 0;
        let mut path_vals = Vec::with_capacity(10); // constraint: [0, 1000]
        let mut path = String::with_capacity(10); // constraint: [0, 1000]
        let mut path_set: HashSet<String> = HashSet::new();
        Self::path_sum_rec(
            &root,
            &mut path,
            &mut path_vals,
            &mut path_set,
            &mut path_sum,
            target_sum,
            '^',
        );
        path_sum
    }

    fn path_sum_rec(
        root: &Option<Rc<RefCell<TreeNode>>>,
        path: &mut String,
        path_vals: &mut Vec<i32>,
        path_set: &mut HashSet<String>,
        path_sum: &mut i32,
        target_sum: i32,
        direction: char,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            println!("visited: {}", node.val);
            path_vals.push(node.val);
            path.push(direction);
            if node.left.is_none() && node.right.is_none() {
                *path_sum += Self::target_sum_count(path, path_vals, path_set, target_sum);
            } else {
                Self::path_sum_rec(
                    &node.left,
                    path,
                    path_vals,
                    path_set,
                    path_sum,
                    target_sum,
                    Self::LEFT,
                );
                Self::path_sum_rec(
                    &node.right,
                    path,
                    path_vals,
                    path_set,
                    path_sum,
                    target_sum,
                    Self::RIGHT,
                );
            }
            path_vals.pop();
            path.pop();
        }
    }

    // https://leetcode.com/problems/subarray-sum-equals-k/description/
    #[inline(always)]
    fn target_sum_count(
        path: &str,
        path_vals: &[i32],
        path_set: &mut HashSet<String>,
        target_sum: i32,
    ) -> i32 {
        println!("search target_sum in: {:?}", path_vals);
        println!("current path        : {:?}", path);
        println!("current path set    : {:?}", path_set);
        let mut cur_sum = 0i32;
        let mut count = 0;
        let mut prefix_sum: HashMap<i32, i32> = HashMap::with_capacity(path_vals.len() + 1);
        prefix_sum.insert(0, 1);
        for (i, &val) in path_vals.iter().enumerate() {
            if let Some(sum) = cur_sum.checked_add(val) {
                cur_sum = sum;
            } else {
                return count;
            }
            if let Some(sum) = prefix_sum.get(&(cur_sum - target_sum)) {
                if !path_set.contains(&path[..=i]) {
                    count += sum;
                    path_set.insert(path[..=i].to_string());
                }
            }
            prefix_sum
                .entry(cur_sum)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        println!("found targets: {count}; path_set: {:?}\n", path_set);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_sum_count_helper() {
        let input = vec![
            1000000000, 1000000000, 294967296, 1000000000, 1000000000, 1000000000,
        ];
        let path = String::from_iter(vec!['<'; input.len()]);
        let mut path_set = HashSet::new();
        assert_eq!(
            0,
            Solution::target_sum_count(&path, &input, &mut path_set, 0)
        );
        assert!(path_set.is_empty());

        let input = vec![1, 10, 6, 7, 11, 3, 9];
        let path = String::from_iter(vec!['<'; input.len()]);
        let mut path_set = HashSet::new();
        assert_eq!(
            2,
            Solution::target_sum_count(&path, &input, &mut path_set, 24)
        );
        let expected = HashSet::from_iter(vec![path[..4].to_string(), path[..5].to_string()]);
        assert_eq!(expected, path_set);

        let input = vec![10, 5, 3, 3];
        let path = String::from_iter(vec!['<'; input.len()]);
        let mut path_set = HashSet::new();
        assert_eq!(
            1,
            Solution::target_sum_count(&path, &input, &mut path_set, 8)
        );
        let expected = HashSet::from_iter(vec![path[..3].to_string()]);
        assert_eq!(expected, path_set);

        let input = vec![1, 1, 1, 1];
        let path = String::from_iter(vec!['<'; input.len()]);
        let mut path_set = HashSet::new();
        assert_eq!(
            4,
            Solution::target_sum_count(&path, &input, &mut path_set, 1)
        );
        let expected = HashSet::from_iter(vec![
            path[..1].to_string(),
            path[..2].to_string(),
            path[..3].to_string(),
            path[..4].to_string(),
        ]);
        assert_eq!(expected, path_set);

        let input = vec![2, 3, 4, 5, 6, 7, 8, 9];
        let path = String::from_iter(vec!['<'; input.len()]);
        let mut path_set = HashSet::new();
        assert_eq!(
            0,
            Solution::target_sum_count(&path, &input, &mut path_set, 100)
        );
        assert!(path_set.is_empty());

        let input = vec![1, 2];
        let path = String::from_iter(vec!['<'; input.len()]);
        let mut path_set = HashSet::new();
        assert_eq!(
            0,
            Solution::target_sum_count(&path, &input, &mut path_set, 0)
        );
        assert!(path_set.is_empty());
    }

    #[test]
    fn path_sum3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        ..Default::default()
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -2,
                        ..Default::default()
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        ..Default::default()
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    ..Default::default()
                }))),
            }))),
        })));
        assert_eq!(3, Solution::path_sum(root, 8));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            right: None,
            left: None,
        })));
        assert_eq!(0, Solution::path_sum(root, 0));
    }
}
