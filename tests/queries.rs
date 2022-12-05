use trie::Trie;

#[test]
fn words_are_complete_unique_and_lexicographically_sorted() {
    let trie: Trie = ["z", "apple", "app", "", "é", "apple"]
        .into_iter()
        .collect();
    assert_eq!(trie.words(), ["", "app", "apple", "z", "é"]);
    assert!(Trie::new().words().is_empty());
}

#[test]
fn autocomplete_returns_only_words_below_the_prefix() {
    let trie: Trie = ["car", "cart", "cat", "dog", ""].into_iter().collect();
    assert_eq!(trie.autocomplete("ca"), ["car", "cart", "cat"]);
    assert_eq!(trie.autocomplete("cart"), ["cart"]);
    assert!(trie.autocomplete("missing").is_empty());
    assert_eq!(trie.autocomplete(""), trie.words());
}
