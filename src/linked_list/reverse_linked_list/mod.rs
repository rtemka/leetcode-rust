struct Solution {}

// https://leetcode.com/problems/reverse-linked-list/description/

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

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur_node = head;
        let mut prev_node: Option<Box<ListNode>> = None;
        while let Some(mut node) = cur_node {
            cur_node = node.next;
            node.next = prev_node.clone();
            // println!("{:?}", node);
            prev_node = Some(node);
        }
        // println!("{:?}", prev_node);
        prev_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_list() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));

        let expected = [4, 3, 2, 1];
        let result = Solution::reverse_list(list);
        let mut cur_node = &result;
        assert!(cur_node.is_some());
        let mut i = 0;
        while let Some(n) = cur_node {
            assert_eq!(expected[i], n.val);
            cur_node = &n.next;
            i += 1;
        }
    }
}
