struct Solution;

use crate::linked_list::ListNode;

impl Solution {
    pub fn reorder_list_rec(_head: &mut Option<Box<ListNode>>) {
        todo!("do recursion version!")
    }

    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut v: Vec<Option<Box<ListNode>>> = Vec::new();
        let mut cur = head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
            v.push(Some(node));
        }
        eprintln!("{:#?}", v);

        let mut cur = head;
        let (mut lo, mut hi) = (0, v.len() - 1);
        let mut i = 0;
        while lo <= hi {
            let k = if i % 2 == 0 { lo } else { hi };
            *cur = v[k].take();
            if let Some(n) = cur {
                cur = &mut n.next;
            }
            (lo, hi) = if i % 2 == 0 {
                (lo + 1, hi)
            } else {
                (lo, hi - 1)
            };
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reorder_list() {
        let mut head = Some(Box::new(ListNode {
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
        let expected = vec![1, 5, 2, 4, 3];

        Solution::reorder_list(&mut head);
        eprintln!("{:#?}", head);

        for n in &expected {
            let node = head.take().expect("expect to take some node");
            assert_eq!(*n, node.val);
            head = node.next;
        }

        Solution::reorder_list_rec(&mut head);
        eprintln!("{:#?}", head);

        for n in expected {
            let node = head.take().expect("expect to take some node");
            assert_eq!(n, node.val);
            head = node.next;
        }
    }
}
