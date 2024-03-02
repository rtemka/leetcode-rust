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

// https://leetcode.com/problems/swap-nodes-in-pairs/description/
struct Solution {}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::swap_pairs_rec(head, true)
    }

    pub fn swap_pairs_rec(head: Option<Box<ListNode>>, swap: bool) -> Option<Box<ListNode>> {
        match head {
            Some(n) if swap => {
                let mut current = n.clone();
                if let Some(mut node) = Self::swap_pairs_rec(n.next, false) {
                    current.next = node.next.clone();
                    node.next = Some(current);
                    Some(node)
                } else {
                    Some(current)
                }
            }
            Some(n) if !swap => {
                let mut current = n.clone();
                if let Some(node) = Self::swap_pairs_rec(n.next, true) {
                    current.next = Some(node);
                    Some(current)
                } else {
                    Some(current)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_pairs() {
        let h = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode { val: 6, next: None })),
                        })),
                    })),
                })),
            })),
        };
        let result_head = Solution::swap_pairs(Some(Box::new(h)));
        let expected = vec![2, 1, 4, 3, 6, 5];
        let mut i = 0;
        assert!(result_head.is_some());
        let mut node = &result_head;
        while let Some(n) = node {
            assert_eq!(expected[i], n.val);
            i += 1;
            node = &n.next;
        }
    }
}
