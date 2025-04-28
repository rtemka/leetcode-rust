// #83
// https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/
struct Solution;

use super::ListNode;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // -100 <= Node.val <= 100
        Self::delete_dups_rec(head, i32::MIN)
    }

    fn delete_dups_rec(head: Option<Box<ListNode>>, prev: i32) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            if node.val == prev {
                Self::delete_dups_rec(node.next, prev)
            } else {
                node.next = Self::delete_dups_rec(node.next, node.val);
                Some(node)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplicates_from_sorted_list() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        }));

        let mut res = Solution::delete_duplicates(head);
        let expected = vec![1,2];
        for exp in expected {
            let node = res.unwrap();
            assert_eq!(node.val, exp);
            res = node.next;
        }
    }
}
