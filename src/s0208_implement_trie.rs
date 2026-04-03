use std::collections::HashMap;

pub struct TrieNode {
    pub children: HashMap<char, Box<TrieNode>>,
    pub is_end: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
        }
    }
}

pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut current = &mut self.root;

        for c in word.chars() {
            current = current
                .children
                .entry(c)
                .or_insert_with(|| Box::new(TrieNode::new()));
        }

        current.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut current = &self.root;

        for c in word.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }

        current.is_end
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.root;

        for c in prefix.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }

        true
    }
}
