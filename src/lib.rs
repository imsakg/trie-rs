extern crate termion;

use termion::color;

use std::collections::HashMap;
pub struct Trie {
    root: TrieNode,
    len: usize,
}

pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TrieNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
            len: 0,
        }
    }
    pub fn insert(&mut self, word: impl AsRef<str>) -> bool {
        let mut current_node = &mut self.root;
        for c in word.as_ref().chars() {
            current_node = current_node.children.entry(c).or_insert(TrieNode::new());
        }
        let added = !current_node.is_word;
        current_node.is_word = true;
        self.len += usize::from(added);
        added
    }

    pub fn contains(&self, word: impl AsRef<str>) -> bool {
        let mut current_node = &self.root;
        for c in word.as_ref().chars() {
            if let Some(node) = current_node.children.get(&c) {
                current_node = node;
            } else {
                return false;
            }
        }
        current_node.is_word
    }

    /// Number of distinct stored words, including the empty word.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether no complete words are stored.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Whether an edge path exists. The empty prefix always exists.
    pub fn has_prefix(&self, prefix: impl AsRef<str>) -> bool {
        let mut node = &self.root;
        for ch in prefix.as_ref().chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return false,
            }
        }
        true
    }

    pub fn print(self) {
        pub fn print_single_node(word: &str, depth: usize, is_word: bool) {
            if depth > 0 {
                let white_space = " ".repeat(depth);
                println!(
                    "{}{} {}",
                    white_space,
                    word,
                    if is_word { "(*)" } else { "" }
                );
            }
        }

        pub fn print_children(node: &TrieNode, depth: usize) {
            for (key, value) in node.children.iter() {
                print_single_node(&key.to_string(), depth, value.is_word);
                print_children(value, depth + 1);
            }
        }

        for (key, value) in self.root.children.iter() {
            println!(
                "{} {}{}",
                color::Fg(color::Red),
                key,
                color::Fg(color::Reset)
            );
            print_single_node(&key.to_string(), 0, value.is_word);
            print_children(value, 1);
        }
    }
}
