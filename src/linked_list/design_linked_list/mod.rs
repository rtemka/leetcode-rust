// https://leetcode.com/problems/design-linked-list/description/
#[derive(Clone, Debug)]
struct MyLinkedList {
    head: Option<Box<Node>>,
}

#[derive(Clone, Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    #[inline]
    fn new() -> Self {
        MyLinkedList { head: None }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }
        let mut cur = &self.head;
        let mut i = 0;
        while let Some(node) = cur {
            if i == index {
                return node.val;
            }
            cur = &node.next;
            i += 1;
        }
        -1
    }

    fn add_at_head(&mut self, val: i32) {
        match &self.head {
            Some(_) => {
                self.head = Some(Box::new(Node {
                    val,
                    next: self.head.take(),
                }));
            }
            None => self.head = Some(Box::new(Node::new(val))),
        }
    }

    fn add_at_tail(&mut self, val: i32) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(val)));
            return;
        }
        let mut cur = &mut self.head;
        while let Some(node) = cur {
            if node.next.is_none() {
                node.next = Some(Box::new(Node::new(val)));
                return;
            }
            cur = &mut node.next;
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if self.head.is_none() && index == 0 {
            self.head = Some(Box::new(Node::new(val)));
            return;
        }
        let mut cur = &mut self.head;
        let mut i = 0;
        while let Some(node) = cur {
            if i == index {
                node.next = Some(node.clone());
                node.val = val;
                return;
            }
            if node.next.is_none() && i + 1 == index {
                node.next = Some(Box::new(Node::new(val)));
                return;
            }
            cur = &mut node.next;
            i += 1;
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        match &mut self.head {
            Some(ref mut n) if index == 0 => self.head = n.next.take(),
            None => return,
            _ => (),
        }
        let mut cur = &mut self.head;
        let mut i = 0;
        while let Some(node) = cur {
            if i + 1 == index {
                node.next = match &mut node.next {
                    Some(n) => n.next.take(),
                    None => None,
                };
                return;
            }
            cur = &mut node.next;
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn design_linked_list_spec_case() {
        let mut list = MyLinkedList::new();
        list.add_at_index(0, 10);
        list.add_at_index(0, 20);
        list.add_at_index(1, 30);
        println!("{:?}", &list);
        assert_eq!(20, list.get(0));
    }

    #[test]
    fn design_linked_list() {
        let mut list = MyLinkedList::new();

        list.add_at_head(0);
        assert_eq!(0, list.get(0));
        list.add_at_tail(1);
        list.add_at_tail(3);
        println!("{:?}", &list);

        assert_eq!(3, list.get(2));
        list.add_at_index(2, 2);
        assert_eq!(2, list.get(2));
        println!("\n\n{:?}", &list);

        list.delete_at_index(2);
        println!("\n\n{:?}", &list);

        list.add_at_index(7, 100);
        assert_eq!(-1, list.get(7));
        println!("\n\n{:?}", &list);

        list.add_at_index(3, 40);
        assert_eq!(40, list.get(3));
        println!("\n\n{:?}", &list);

        list.add_at_head(100);
        assert_eq!(100, list.get(0));
        println!("\n\n{:?}", &list);
    }
}
