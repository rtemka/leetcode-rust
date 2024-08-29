pub mod binary_tree_postorder_traversal;
pub mod binary_tree_right_side_view;
pub mod count_good_nodes_in_bst;
pub mod delete_node_in_bst;
pub mod leaf_similar_trees;
pub mod longest_zigzag_path_in_binary_tree;
pub mod max_depth_of_binary_tree;
pub mod path_sum;
pub mod path_sum_2;
pub mod unique_binary_search_trees;
pub mod unique_binary_search_trees_2;
pub mod validate_bst;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
