pub mod binary_tree_inorder_traversal;
pub mod binary_tree_iterator;
pub mod binary_tree_postorder_traversal;
pub mod binary_tree_right_side_view;
pub mod count_good_nodes_in_bst;
pub mod delete_node_in_bst;
pub mod insert_into_bst;
pub mod invert_binary_tree;
pub mod leaf_similar_trees;
pub mod longest_zigzag_path_in_binary_tree;
pub mod lowest_common_ancestor_of_binary_tree;
pub mod lowest_common_ancestor_of_deepest_leaves;
pub mod max_depth_of_binary_tree;
pub mod max_level_sum_of_binary_tree;
pub mod minimum_depth_of_binary_tree;
pub mod path_sum;
pub mod path_sum_2;
pub mod path_sum_3;
pub mod same_tree;
pub mod unique_binary_search_trees;
pub mod unique_binary_search_trees_2;
pub mod validate_bst;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Default)]
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

// impl PartialEq for TreeNode {
//     fn eq(&self, other: &Self) -> bool {
//         self.val == other.val && self.left == other.left && self.right == other.right
//     }
// }
