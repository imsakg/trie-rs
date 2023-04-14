use trie::Trie;

#[test]
fn insensitive_mode_normalizes_insertions_and_exact_lookups() {
    let mut trie = Trie::case_insensitive();
    assert!(trie.is_case_insensitive());
    assert!(!Trie::new().is_case_insensitive());
    assert!(trie.insert("Rust"));
    assert!(!trie.insert("RUST"));
    assert!(trie.contains("rUsT"));
    assert_eq!(trie.words(), ["rust"]);
    assert_eq!(trie.len(), 1);
}
