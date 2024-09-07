// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/description
struct Solution;

use crate::linked_list::ListNode;

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut vals = Vec::new();
        let mut cur = &head;
        while let Some(n) = cur {
            vals.push(n.val);
            cur = &n.next;
        }
        let (mut lo, mut hi) = (0, vals.len() - 1);
        let mut max_sum = 0;
        while lo < hi {
            max_sum = max_sum.max(vals[lo] + vals[hi]);
            (lo, hi) = (lo + 1, hi - 1);
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_twin_sum() {
        let head = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));
        assert_eq!(6, Solution::pair_sum(head));
    }
}
