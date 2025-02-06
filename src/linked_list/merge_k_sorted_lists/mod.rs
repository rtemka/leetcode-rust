// https://leetcode.com/problems/merge-k-sorted-lists/description/
struct Solution;

use super::ListNode;

use std::{cmp, collections::BinaryHeap};

#[derive(Debug, PartialEq, Eq)]
struct MinNode {
    val: i32,
    idx: usize,
}

impl MinNode {
    #[inline]
    fn new(val: i32, idx: usize) -> Self {
        Self { val, idx }
    }
}

impl Ord for MinNode {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.val.cmp(&self.val) // note: the reverse!
    }
}

impl PartialOrd for MinNode {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(other.cmp(self)) // note: the reverse!
    }
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        other.val < self.val
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        other.val <= self.val
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        other.val > self.val
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        other.val >= self.val
    }
}

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // First: we construct the priority queue to keep
        // track of the min element across the linked lists.
        let mut heap: BinaryHeap<MinNode> = BinaryHeap::with_capacity(lists.len());

        for (idx, l) in lists.iter().enumerate() {
            if let Some(node) = l {
                heap.push(MinNode::new(node.val, idx));
            }
        }
        // dbg!(&heap);

        // Create the fake head node to save pointer to the first element.
        let mut head = ListNode::new(0);

        // This will be our work pointer during list construction.
        let mut cur = &mut head.next;

        // While there are some elements in the queue
        while let Some(min_node) = heap.pop() {
            // println!("popped node val: {}", min_node.val);

            // Take head of the current min list in the array.
            if let Some(mut node) = lists[min_node.idx].take() {
                // Detach the next node from it(if any).
                let next = node.next.take();
                // Push it's value to the queue.
                if let Some(ref node) = next {
                    heap.push(MinNode::new(node.val, min_node.idx));
                }
                // Insert the current min node to the result.
                *cur = Some(node);
                if let Some(n) = cur {
                    // Shift the pointer.
                    cur = &mut n.next;
                }
                // Return the next node back into the array of lists.
                lists[min_node.idx] = next;
            }
        }
        head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_k_sorted_lists() {
        let lists = vec![
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 6, next: None })),
            })),
        ];
        let mut got = Solution::merge_k_lists(lists);
        let expected = vec![1, 1, 2, 3, 4, 4, 5, 6];
        for n in expected {
            let node = got.take().expect("expect node, got none");
            assert_eq!(node.val, n);
            got = node.next;
        }
    }
}
