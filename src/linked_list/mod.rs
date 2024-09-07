pub mod add_two_numbers;
pub mod delete_nodes_in_linked_list_present_in_array;
pub mod delete_the_middle_node;
pub mod design_linked_list;
pub mod max_twin_sum_of_linked_list;
pub mod odd_even_linked_list;
pub mod palindrome_linked_list;
pub mod remove_linked_list_elements;
pub mod remove_nth_node_from_end_of_list;
pub mod rotate_list;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
