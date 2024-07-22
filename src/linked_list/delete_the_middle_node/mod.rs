// https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list
struct Solution;

use crate::linked_list::ListNode;
impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list_len = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            list_len += 1;
            cur = &node.next;
        }
        let middle = list_len / 2;
        if middle == 0 {
            return head.unwrap().next;
        }
        let mut i = 0;
        let mut cur = &mut head;
        while let Some(node) = cur {
            if i + 1 == middle {
                if let Some(n) = node.next.take() {
                    node.next = n.next;
                }
                break;
            }
            i += 1;
            cur = &mut node.next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_middle_node() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 7,
                        next: Some(Box::new(ListNode {
                            val: 1,
                            next: Some(Box::new(ListNode {
                                val: 2,
                                next: Some(Box::new(ListNode { val: 6, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let expected = vec![1, 3, 4, 1, 2, 6];
        let mut ans = Solution::delete_middle(list);
        for n in expected {
            let node = ans.unwrap();
            assert_eq!(n, node.val);
            ans = node.next;
        }
    }
}
