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

#[test]
fn borrowed_strings_are_accepted_without_allocating_at_call_sites() {
    let mut trie = Trie::new();
    let owned = String::from("borrowed");
    trie.insert(&owned);
    assert!(trie.contains("borrowed"));
    assert!(trie.contains(&owned));
}
