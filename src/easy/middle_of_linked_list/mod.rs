use std::ops::Deref;

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

struct Solution {}

fn size(acc: i32, n: Option<&Box<ListNode>>) -> i32 {
    if let Some(n) = n {
        return size(acc + 1, n.next.as_ref());
    }
    acc
}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // let size = size(0, head.as_ref());
        let mut count = 0;
        let mut node = &head;
        while let Some(n) = node {
            count += 1;
            node = &n.deref().next;
        }
        let middle = count / 2 + 1;
        count = 1;
        node = &head;
        while let Some(n) = node {
            if count == middle {
                return node.clone();
            }
            count += 1;
            node = &n.deref().next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    #[test]
    fn middle_of_the_linked_list() {
        let head2 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        };

        assert_eq!(
            2,
            Solution::middle_node(Some(Box::new(head2)))
                .unwrap()
                .deref()
                .val
        );
    }
}
