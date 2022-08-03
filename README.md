# trie-rs

A character trie for a set of UTF-8 strings. The Cargo package and Rust crate are
named `trie`; the repository is `trie-rs`.

```rust
use trie::Trie;

let mut words = Trie::new();
assert!(words.insert("rust"));
assert!(!words.insert("rust")); // set semantics
assert!(words.contains("rust"));
assert!(!words.contains("rus"));
assert!(words.has_prefix("rus"));
assert_eq!(words.len(), 1);
```

Words may include the empty string, spaces, punctuation, and non-ASCII characters.
Matching is case-sensitive by default. A prefix path is not necessarily a stored
word. The empty prefix is a valid path, even in an empty trie.

Run the library tests with `cargo test`. See [the implementation plan](docs/development-plan.md)
for the remaining work and [history provenance](HISTORY.md) for timestamp context.
