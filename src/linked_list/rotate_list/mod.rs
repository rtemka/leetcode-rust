use crate::linked_list::ListNode;

// https://leetcode.com/problems/rotate-list/description/
struct Solution;

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // calculate the length of list.
        let mut len = 0;
        let mut cur = &mut head;
        while let Some(n) = cur {
            len += 1;
            cur = &mut n.next;
        }

        // return if there is nothing to do.
        if len == 0 || len == k || k % len == 0 {
            return head;
        }

        // point where we must break list.
        let unlink = len - k % len;
        let mut i = 1;
        let mut new_head = None;
        cur = &mut head;
        while let Some(n) = cur {
            if i == unlink {
                new_head = n.next.take();
                break;
            } else {
                cur = &mut n.next;
            }
            i += 1;
        }

        // now go to the tail of new_head and
        // attach the previous head to it.
        cur = &mut new_head;
        while let Some(n) = cur {
            if n.next.is_none() {
                n.next = head.take();
                break;
            }
            cur = &mut n.next;
        }
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_right() {
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
        let mut ans = Solution::rotate_right(Some(Box::new(head)), 2);
        for i in vec![4, 5, 1, 2, 3] {
            let node = ans.unwrap();
            assert_eq!(i, node.val);
            ans = node.next;
        }

        let head = ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        };
        let mut ans = Solution::rotate_right(Some(Box::new(head)), 4);
        for i in vec![2, 0, 1] {
            let node = ans.unwrap();
            assert_eq!(i, node.val);
            ans = node.next;
        }
    }
}
