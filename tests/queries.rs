use trie::Trie;

#[test]
fn words_are_complete_unique_and_lexicographically_sorted() {
    let trie: Trie = ["z", "apple", "app", "", "é", "apple"]
        .into_iter()
        .collect();
    assert_eq!(trie.words(), ["", "app", "apple", "z", "é"]);
    assert!(Trie::new().words().is_empty());
}
