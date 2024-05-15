use crate::linked_list::ListNode;

// https://leetcode.com/problems/palindrome-linked-list/description/
struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        Self::is_palindrome_helper(&head, &head).is_ok()
    }

    fn is_palindrome_helper<'a>(
        tail: &'a Option<Box<ListNode>>,
        head: &'a Option<Box<ListNode>>,
    ) -> Result<&'a Option<Box<ListNode>>, ()> {
        match tail {
            Some(n) => match Self::is_palindrome_helper(&n.next, head)? {
                Some(hn) => {
                    if n.val == hn.val {
                        Ok(&hn.next)
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            },
            None => Ok(head),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_is_palindrome() {
        let head = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode { val: 1, next: None })),
                        })),
                    })),
                })),
            })),
        };

        assert!(Solution::is_palindrome(Some(Box::new(head))));

        assert!(!Solution::is_palindrome(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None }))
        }))));
        // assert!(!Solution::is_palindrome(Some(Box::new(head))));

        assert!(Solution::is_palindrome(Some(Box::new(ListNode {
            val: 1,
            next: None
        }))));
    }
}
