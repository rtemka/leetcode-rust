// https://leetcode.com/problems/remove-linked-list-elements/description/
struct Solution;

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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head {
            Some(mut n) => match Self::remove_elements(n.next, val) {
                Some(nn) if n.val == val => Some(nn),
                None if n.val == val => None,
                Some(nn) => {
                    n.next = Some(nn);
                    Some(n)
                }
                None => {
                    n.next = None;
                    Some(n)
                }
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_ll_elems() {
        let head = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        };
        let answer = Solution::remove_elements(Some(Box::new(head)), 2);
        println!("{:?}", answer);
        let mut cur = &answer;
        while let Some(node) = cur {
            assert!(node.val != 2);
            cur = &node.next;
        }
    }
}
