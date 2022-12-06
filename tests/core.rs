use trie::Trie;

#[test]
fn insertion_marks_only_whole_words() {
    let mut trie = Trie::new();
    trie.insert("rust");
    assert!(trie.contains("rust"));
    assert!(!trie.contains("rus"));
    assert!(!trie.contains("rusty"));
}

#[test]
fn default_trie_is_empty_and_case_sensitive() {
    let mut trie = Trie::default();
    assert!(!trie.contains(String::new()));
    trie.insert("Tree");
    assert!(!trie.contains("tree"));
}

#[test]
fn borrowed_strings_are_accepted_without_allocating_at_call_sites() {
    let mut trie = Trie::new();
    let owned = String::from("borrowed");
    trie.insert(&owned);
    assert!(trie.contains("borrowed"));
    assert!(trie.contains(&owned));
}

#[test]
fn length_counts_unique_words_and_insert_reports_changes() {
    let mut trie = Trie::new();
    assert!(trie.is_empty());
    assert!(trie.insert("one"));
    assert!(!trie.insert("one"));
    assert!(trie.insert(""));
    assert_eq!(trie.len(), 2);
    assert!(!trie.is_empty());
}

#[test]
fn unicode_empty_and_punctuation_words_are_not_filtered() {
    let mut trie = Trie::new();
    for word in ["", "éclair", "東京", "🦀", "a-b", "line\nbreak", "42"] {
        trie.insert(word);
        assert!(trie.contains(word));
    }
    assert_eq!(trie.len(), 7);
    assert!(!trie.contains("écla"));
    assert!(!trie.contains("東"));
}

#[test]
fn prefix_search_distinguishes_paths_from_words() {
    let mut trie = Trie::new();
    assert!(trie.has_prefix(""));
    assert!(!trie.has_prefix("ca"));
    trie.insert("cat");
    assert!(trie.has_prefix("ca"));
    assert!(!trie.contains("ca"));
    assert!(trie.has_prefix("cat"));
    assert!(!trie.has_prefix("cats"));
}

#[test]
fn remove_reports_membership_and_keeps_other_words() {
    let mut trie = Trie::new();
    for word in ["car", "cart", "cat", ""] {
        trie.insert(word);
    }
    assert!(!trie.remove("ca"));
    assert!(trie.remove("car"));
    assert!(!trie.contains("car"));
    assert!(trie.contains("cart"));
    assert_eq!(trie.len(), 3);
    assert!(trie.remove("cart"));
    assert!(!trie.has_prefix("car"));
    assert!(trie.contains("cat"));
    assert!(trie.remove(""));
    assert!(!trie.remove(""));
    assert_eq!(trie.len(), 1);
}

#[test]
fn deleting_a_longer_word_preserves_its_terminal_prefix() {
    let mut trie = Trie::new();
    trie.insert("🦀");
    trie.insert("🦀rust");
    assert!(trie.remove("🦀rust"));
    assert!(trie.contains("🦀"));
    assert!(!trie.has_prefix("🦀r"));
    assert!(trie.remove("🦀"));
    assert!(trie.is_empty());
    assert!(!trie.has_prefix("🦀"));
    assert!(!trie.remove("missing"));
}

#[test]
fn clear_discards_words_and_all_prefix_edges() {
    let mut trie = Trie::new();
    trie.insert("clear");
    trie.insert("");
    trie.clear();
    assert!(trie.is_empty());
    assert!(!trie.contains(""));
    assert!(!trie.has_prefix("c"));
    assert!(trie.insert("again"));
    assert_eq!(trie.len(), 1);
}

#[test]
fn extend_accepts_borrowed_batches_with_set_semantics() {
    let mut trie = Trie::new();
    trie.extend(["a", "b", "a"]);
    assert_eq!(trie.len(), 2);
    assert!(trie.contains("a"));
    trie.extend(vec![String::from("c")]);
    assert!(trie.contains("c"));
}

#[test]
fn collect_builds_a_trie_from_any_string_iterator() {
    let trie: Trie = ["rain", "rainbow", "rain"].into_iter().collect();
    assert_eq!(trie.len(), 2);
    assert!(trie.has_prefix("rainb"));
    let empty: Trie = std::iter::empty::<String>().collect();
    assert!(empty.is_empty());
}

#[test]
fn cloning_a_trie_produces_an_independent_dictionary() {
    let original: Trie = ["apple", "pear"].into_iter().collect();
    let mut copy = original.clone();
    copy.remove("apple");
    copy.insert("orange");
    assert!(original.contains("apple"));
    assert!(!original.contains("orange"));
    assert!(format!("{copy:?}").contains("Trie"));
}
