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

#[test]
fn insensitive_mode_applies_to_prefix_queries_and_removal() {
    let mut trie = Trie::case_insensitive();
    trie.extend(["Cart", "CAR", "Cat"]);
    assert!(trie.has_prefix("CA"));
    assert_eq!(trie.autocomplete("Ca"), ["car", "cart", "cat"]);
    assert_eq!(trie.count_prefix("CA"), 3);
    assert_eq!(trie.longest_prefix("CARTwheel").as_deref(), Some("cart"));
    assert!(trie.remove("cAr"));
    assert!(!trie.contains("CAR"));
    trie.clear();
    assert!(trie.is_case_insensitive());
    trie.insert("AGAIN");
    assert!(trie.contains("again"));
}

#[test]
fn lowercase_semantics_are_unicode_but_not_full_case_folding() {
    let mut trie = Trie::case_insensitive();
    trie.extend(["ÉCLAIR", "İ", "Straße", "Σ"]);
    assert!(trie.contains("éclair"));
    assert!(trie.contains("i\u{307}"));
    assert!(!trie.contains("i"));
    assert!(!trie.contains("STRASSE"));
    assert!(trie.contains("σ"));
    assert!(!trie.contains("ς"));
    assert_eq!(trie.longest_prefix("İstanbul").as_deref(), Some("i\u{307}"));
}
