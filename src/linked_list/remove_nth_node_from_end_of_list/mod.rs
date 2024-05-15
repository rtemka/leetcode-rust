use crate::linked_list::ListNode;

struct Solution;

// https://leetcode.com/problems/remove-nth-node-from-end-of-list/description/
impl Solution {
    // O(1) on memory, O(n) on time
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = &mut head;
        while let Some(node) = cur {
            len += 1;
            cur = &mut node.next;
        }
        let mut i = 0;
        cur = &mut head;
        while let Some(node) = cur {
            // if this is head itself.
            if i == len - n {
                return node.next.clone();
            }
            // case when it's some next node.
            if i + 1 == len - n {
                if let Some(next_node) = &node.next {
                    node.next = next_node.next.clone();
                } else {
                    node.next = None;
                }
                return head;
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
    fn remove_nth_from_end() {
        let head = ListNode {
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
        };
        let answer = Solution::remove_nth_from_end(Some(Box::new(head)), 2);
        let mut n = answer;
        while let Some(node) = n {
            assert!(node.val != 4);
            n = node.next;
        }
        let head = ListNode::new(1);
        let answer = Solution::remove_nth_from_end(Some(Box::new(head)), 1);
        assert!(answer.is_none());
        let head = ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        };
        let answer = Solution::remove_nth_from_end(Some(Box::new(head)), 1).unwrap();
        assert!(answer.val == 1);
        assert!(answer.next.is_none());
    }
}
