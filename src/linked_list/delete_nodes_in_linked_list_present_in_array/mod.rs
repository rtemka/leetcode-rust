// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/description
struct Solution;

use crate::linked_list::ListNode;

impl Solution {
    pub fn modified_list(mut nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        nums.sort_unstable();
        Self::modified_list_rec(nums, head)
    }

    pub fn modified_list_2(
        mut nums: Vec<i32>,
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        nums.sort_unstable();
        let mut cur = &mut head;
        while let Some(ref mut node) = cur {
            while nums.binary_search(&node.val).is_ok() {
                if let Some(n) = node.next.take() {
                    *node = n;
                }
            }
            cur = &mut node.next;
        }
        head
    }

    pub fn modified_list_rec(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => {
                while nums.binary_search(&node.val).is_ok() {
                    node = node.next.take()?;
                }
                node.next = Self::modified_list_rec(nums, node.next);
                Some(node)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_nodes_present_in_array() {
        let to_delete = vec![1, 2, 3];
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));

        let ans = Solution::modified_list(to_delete, head);
        assert!(ans.is_some());
        let mut cur = &ans;
        let expected = vec![4, 5];
        let mut i = 0;
        while let Some(n) = cur {
            assert_eq!(expected[i], n.val);
            cur = &n.next;
            i += 1;
        }
    }
}
