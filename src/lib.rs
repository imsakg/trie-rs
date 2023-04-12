extern crate termion;

use termion::color;

use std::collections::BTreeMap;
#[derive(Clone, Debug)]
pub struct Trie {
    root: TrieNode,
    len: usize,
}

#[derive(Clone, Debug)]
pub struct TrieNode {
    children: BTreeMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: BTreeMap::new(),
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

impl<S: AsRef<str>> Extend<S> for Trie {
    fn extend<I: IntoIterator<Item = S>>(&mut self, iter: I) {
        for word in iter {
            self.insert(word);
        }
    }
}

impl<S: AsRef<str>> FromIterator<S> for Trie {
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        let mut trie = Self::new();
        trie.extend(iter);
        trie
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
            current_node = current_node.children.entry(c).or_default();
        }
        let added = !current_node.is_word;
        current_node.is_word = true;
        self.len += usize::from(added);
        added
    }

    pub fn contains(&self, word: impl AsRef<str>) -> bool {
        self.node(word.as_ref()).is_some_and(|node| node.is_word)
    }

    fn node(&self, word: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for ch in word.chars() {
            node = node.children.get(&ch)?;
        }
        Some(node)
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
        self.node(prefix.as_ref()).is_some()
    }

    /// Remove a complete word and prune edges no longer used by another word.
    pub fn remove(&mut self, word: impl AsRef<str>) -> bool {
        fn remove_at(node: &mut TrieNode, chars: &mut std::str::Chars<'_>) -> bool {
            match chars.next() {
                None => std::mem::replace(&mut node.is_word, false),
                Some(ch) => {
                    let (removed, prune) = match node.children.get_mut(&ch) {
                        Some(child) => {
                            let removed = remove_at(child, chars);
                            (removed, !child.is_word && child.children.is_empty())
                        }
                        None => return false,
                    };
                    if prune {
                        node.children.remove(&ch);
                    }
                    removed
                }
            }
        }
        let removed = remove_at(&mut self.root, &mut word.as_ref().chars());
        self.len -= usize::from(removed);
        removed
    }

    /// Discard all words while retaining the trie's configuration.
    pub fn clear(&mut self) {
        self.root = TrieNode::new();
        self.len = 0;
    }

    /// Return all stored words in Unicode scalar lexicographic order.
    pub fn words(&self) -> Vec<String> {
        self.autocomplete("")
    }

    /// Return complete words that start with `prefix`, in lexical order.
    pub fn autocomplete(&self, prefix: impl AsRef<str>) -> Vec<String> {
        self.autocomplete_with_limit(prefix, usize::MAX)
    }

    /// Return at most `limit` completions, stopping traversal when it is reached.
    pub fn autocomplete_with_limit(&self, prefix: impl AsRef<str>, limit: usize) -> Vec<String> {
        if limit == 0 {
            return Vec::new();
        }
        let prefix = prefix.as_ref();
        let Some(root) = self.node(prefix) else {
            return Vec::new();
        };
        let mut words = Vec::new();
        let mut stack = vec![(root, prefix.to_owned())];
        while let Some((node, word)) = stack.pop() {
            if node.is_word {
                words.push(word.clone());
                if words.len() == limit {
                    break;
                }
            }
            for (ch, child) in node.children.iter().rev() {
                let mut next = word.clone();
                next.push(*ch);
                stack.push((child, next));
            }
        }
        words
    }

    /// Count complete words in a prefix subtree without allocating result strings.
    pub fn count_prefix(&self, prefix: impl AsRef<str>) -> usize {
        let Some(root) = self.node(prefix.as_ref()) else {
            return 0;
        };
        let mut stack = vec![root];
        let mut count = 0;
        while let Some(node) = stack.pop() {
            count += usize::from(node.is_word);
            stack.extend(node.children.values());
        }
        count
    }

    /// Longest stored word that is a prefix of the input.
    pub fn longest_prefix(&self, input: impl AsRef<str>) -> Option<String> {
        let input = input.as_ref();
        let mut node = &self.root;
        let mut end = node.is_word.then_some(0);
        for (index, ch) in input.char_indices() {
            let Some(child) = node.children.get(&ch) else {
                break;
            };
            node = child;
            if node.is_word {
                end = Some(index + ch.len_utf8());
            }
        }
        end.map(|end| input[..end].to_owned())
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
