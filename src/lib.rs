extern crate termion;

use termion::color;

use std::collections::HashMap;
pub struct Trie {
    root: TrieNode,
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
        }
    }
    pub fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            current_node = current_node.children.entry(c).or_insert(TrieNode::new());
        }
        current_node.is_word = true;
    }

    pub fn contains(&self, word: String) -> bool {
        let mut current_node = &self.root;
        for c in word.chars() {
            if let Some(node) = current_node.children.get(&c) {
                current_node = node;
            } else {
                return false;
            }
        }
        current_node.is_word
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
