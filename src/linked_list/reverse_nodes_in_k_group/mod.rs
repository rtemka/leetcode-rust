// https://leetcode.com/problems/reverse-nodes-in-k-group/description/
struct Solution;

use crate::linked_list::ListNode;

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // gather node values into array
        let mut nodes: Vec<i32> = Vec::new();
        let mut cur = &head;
        while let Some(node) = cur {
            nodes.push(node.val);
            cur = &node.next;
        }

        // swap node values in the array
        let k = k as usize;
        let mut i = 0;
        while i + k <= nodes.len() {
            nodes[i..i + k].reverse();
            i += k;
        }

        // write swaped values into linked list
        let mut cur = &mut head;
        let mut i = 0;
        while let Some(node) = cur {
            node.val = nodes[i];
            cur = &mut node.next;
            i += 1;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_k_group() {
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

        let expected = vec![2, 1, 4, 3, 5];
        let mut ans = Solution::reverse_k_group(head, 2);
        for num in expected {
            let node = ans.take().unwrap();
            assert_eq!(node.val, num);
            ans = node.next;
        }
    }
}
