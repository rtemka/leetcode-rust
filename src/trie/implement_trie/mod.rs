// #208
// https://leetcode.com/problems/implement-trie-prefix-tree/description/
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
struct Trie {
    root: Rc<RefCell<TrieNode>>,
}

#[derive(Debug)]
struct TrieNode {
    children: HashMap<char, Rc<RefCell<TrieNode>>>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(TrieNode::new())),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = self.root.clone();
        for c in word.chars() {
            {
                node.borrow_mut()
                    .children
                    .entry(c)
                    .or_insert(Rc::new(RefCell::new(TrieNode::new())));
            }
            if let Some(n) = node.clone().borrow().children.get(&c) {
                node = n.clone();
            }
        }
        node.borrow_mut().is_word = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = self.root.clone();
        for c in word.chars() {
            node = match node.clone().borrow().children.get(&c) {
                Some(n) => n.clone(),
                None => return false,
            };
        }
        let is_word = node.borrow().is_word;
        is_word == true
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = self.root.clone();
        for c in prefix.chars() {
            node = match node.clone().borrow().children.get(&c) {
                Some(n) => n.clone(),
                None => return false,
            };
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implement_the_trie() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        dbg!(&trie);
        assert!(trie.search("app".to_string()));
    }
}
