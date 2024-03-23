// https://leetcode.com/problems/merge-two-sorted-lists/description/
struct Solution {}

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut cur = &mut head;
        let (mut left, mut right) = (&list1, &list2);
        loop {
            match (left, right) {
                (Some(l), Some(r)) if l.val > r.val => {
                    if let Some(n) = cur {
                        n.next = right.clone();
                        cur = &mut n.next;
                    } else {
                        *cur = right.clone();
                    }
                    right = &r.next;
                }
                (Some(l), Some(r)) if r.val >= l.val => {
                    if let Some(n) = cur {
                        n.next = left.clone();
                        cur = &mut n.next;
                    } else {
                        *cur = left.clone();
                    }
                    left = &l.next;
                }
                (None, Some(r)) => {
                    if let Some(n) = cur {
                        n.next = right.clone();
                        cur = &mut n.next;
                    } else {
                        *cur = right.clone();
                    }
                    right = &r.next;
                }
                (Some(l), None) => {
                    if let Some(n) = cur {
                        n.next = left.clone();
                        cur = &mut n.next;
                    } else {
                        *cur = left.clone();
                    }
                    left = &l.next;
                }
                _ => break,
            }
        }
        head
    }

    pub fn merge_two_lists_rec(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(l), None) | (None, Some(l)) => Some(l),
            (None, None) => None,
            (Some(l1), Some(l2)) => match l1.val <= l2.val {
                true => Some(Box::new(ListNode {
                    val: l1.val,
                    next: Self::merge_two_lists(l1.next, Some(l2)),
                })),
                false => Some(Box::new(ListNode {
                    val: l2.val,
                    next: Self::merge_two_lists(Some(l1), l2.next),
                })),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_two_sorted_lists() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let expected = vec![1, 1, 2, 3, 4, 4];
        let result = Solution::merge_two_lists(list1, list2);
        assert!(result.is_some());
        let mut head = &result;
        let mut i = 0;
        while let Some(n) = head {
            println!("expected:{}\tgot:{}", expected[i], n.val);
            assert_eq!(expected[i], n.val);
            i += 1;
            head = &n.next;
        }
    }
}
