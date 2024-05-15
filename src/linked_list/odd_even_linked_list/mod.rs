use crate::linked_list::ListNode;

// https://leetcode.com/problems/odd-even-linked-list/description/
struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut odd, mut even) = (head, Box::new(ListNode::new(-1)));
        let (mut e, mut o) = (&mut even, &mut odd);

        while let Some(n) = o {
            e.next = n.next.take();
            // println!("{n:?}\n{:?}", e);
            if let Some(ref mut nn) = e.next {
                n.next = nn.next.take();
                e = nn;
            }
            if n.next.is_some() {
                o = &mut n.next;
            } else {
                n.next = even.next;
                break;
            }
        }
        odd
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_even_list() {
        let head = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode { val: 7, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        };

        let mut answer = Solution::odd_even_list(Some(Box::new(head)));
        println!("{answer:?}");
        for n in vec![1, 3, 5, 7, 2, 4, 6] {
            println!("{n}");
            let node = answer.unwrap();
            assert_eq!(n, node.val);
            answer = node.next;
        }
    }
}
