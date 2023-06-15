use trie::Trie;

#[test]
fn insertion_marks_only_whole_words() {
    let mut trie = Trie::new();
    trie.insert("rust".to_string());
    assert!(trie.contains("rust".to_string()));
    assert!(!trie.contains("rus".to_string()));
    assert!(!trie.contains("rusty".to_string()));
}

#[test]
fn default_trie_is_empty_and_case_sensitive() {
    let mut trie = Trie::default();
    assert!(!trie.contains(String::new()));
    trie.insert("Tree".to_string());
    assert!(!trie.contains("tree".to_string()));
}
