use crate::linked_list::ListNode;

// https://leetcode.com/problems/add-two-numbers/description/
struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut head;
        let mut carry = 0;
        let (mut l1_ref, mut l2_ref) = (&l1, &l2);
        loop {
            let val = match (&l1_ref, &l2_ref) {
                (Some(n1), Some(n2)) => {
                    l1_ref = &n1.next;
                    l2_ref = &n2.next;
                    n1.val + n2.val + carry
                }
                (None, Some(n2)) => {
                    l2_ref = &n2.next;
                    n2.val + carry
                }
                (Some(n1), None) => {
                    l1_ref = &n1.next;
                    n1.val + carry
                }
                (None, None) => -1,
            };
            if let Some(n) = cur {
                let val = match val {
                    0..=9 => {
                        carry = 0;
                        val
                    }
                    10.. => {
                        carry = 1;
                        val - 10
                    }
                    _ if carry > 0 => {
                        carry = 0;
                        1
                    }
                    _ => break,
                };
                n.next = Some(Box::new(ListNode::new(val)));
                cur = &mut n.next;
            }
        }
        head.unwrap().next
    }
}

//   9 9 9 9 9
//       7 8 9
// 1 0 0 7 8 8

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_list_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let mut answer = Solution::add_two_numbers(l1, l2);
        for i in [7, 0, 8] {
            let n = answer.expect("expect not None value");
            assert_eq!(i, n.val);
            answer = n.next;
        }

        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode { val: 9, next: None }));
        let mut answer = Solution::add_two_numbers(l1, l2);
        for i in [1, 5, 3] {
            let n = answer.expect("expect not None value");
            assert_eq!(i, n.val);
            answer = n.next;
        }

        let l1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode { val: 9, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode { val: 9, next: None })),
            })),
        }));
        let mut answer = Solution::add_two_numbers(l1, l2);
        for i in [8, 9, 9, 1] {
            let n = answer.expect("expect not None value");
            assert_eq!(i, n.val);
            answer = n.next;
        }
    }
}
