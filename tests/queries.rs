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

#[test]
fn completion_limits_are_sorted_and_zero_is_empty() {
    let trie: Trie = ["ab", "a", "ac", "b"].into_iter().collect();
    assert_eq!(trie.autocomplete_with_limit("a", 2), ["a", "ab"]);
    assert!(trie.autocomplete_with_limit("a", 0).is_empty());
    assert_eq!(
        trie.autocomplete_with_limit("a", usize::MAX),
        ["a", "ab", "ac"]
    );
}

#[test]
fn count_prefix_includes_terminal_prefix_and_empty_word() {
    let trie: Trie = ["", "a", "ab", "abc", "b"].into_iter().collect();
    assert_eq!(trie.count_prefix(""), 5);
    assert_eq!(trie.count_prefix("a"), 3);
    assert_eq!(trie.count_prefix("ab"), 2);
    assert_eq!(trie.count_prefix("z"), 0);
}

#[test]
fn longest_prefix_returns_a_word_not_just_a_path() {
    let mut trie: Trie = ["a", "app", "apple", "é"].into_iter().collect();
    assert_eq!(trie.longest_prefix("applesauce").as_deref(), Some("apple"));
    assert_eq!(trie.longest_prefix("apt").as_deref(), Some("a"));
    assert_eq!(trie.longest_prefix("éclair").as_deref(), Some("é"));
    assert_eq!(trie.longest_prefix("z"), None);
    trie.insert("");
    assert_eq!(trie.longest_prefix("z").as_deref(), Some(""));
}

#[test]
fn tree_rendering_is_plain_deterministic_and_non_consuming() {
    let trie: Trie = ["b", "a", "ab", ""].into_iter().collect();
    assert_eq!(trie.tree(), "(*)\na (*)\n b (*)\nb (*)\n");
    assert_eq!(trie.len(), 4);
    assert_eq!(trie.tree(), trie.tree());
}

#[test]
fn question_mark_matches_exactly_one_unicode_scalar() {
    let trie: Trie = ["cat", "cot", "ct", "c🦀t", "cats", ""]
        .into_iter()
        .collect();
    assert_eq!(trie.wildcard("c?t"), ["cat", "cot", "c🦀t"]);
    assert_eq!(trie.wildcard(""), [""]);
    assert_eq!(trie.wildcard("cat"), ["cat"]);
    assert_eq!(trie.wildcard("??"), ["ct"]);
}

#[test]
fn star_matches_zero_or_more_scalars_without_duplicate_results() {
    let trie: Trie = ["", "a", "ab", "abb", "acb", "ba", "🦀"]
        .into_iter()
        .collect();
    assert_eq!(trie.wildcard("a*b"), ["ab", "abb", "acb"]);
    assert_eq!(trie.wildcard("**"), trie.words());
    assert_eq!(trie.wildcard("a*"), ["a", "ab", "abb", "acb"]);
    assert_eq!(trie.wildcard("*?"), ["a", "ab", "abb", "acb", "ba", "🦀"]);
}

#[test]
fn escaped_wildcards_and_backslashes_match_literal_characters() {
    let trie: Trie = ["a*b", "a?b", r"a\b", "axb", r"a\"].into_iter().collect();
    assert_eq!(trie.wildcard(r"a\*b"), ["a*b"]);
    assert_eq!(trie.wildcard(r"a\?b"), ["a?b"]);
    assert_eq!(trie.wildcard(r"a\\b"), [r"a\b"]);
    assert_eq!(trie.wildcard(r"a\"), [r"a\"]);
}

#[test]
fn wildcard_limits_select_the_first_sorted_matches() {
    let trie: Trie = ["a", "ab", "abc", "b"].into_iter().collect();
    assert_eq!(trie.wildcard_with_limit("*", 2), ["a", "ab"]);
    assert!(trie.wildcard_with_limit("*", 0).is_empty());
    assert_eq!(trie.wildcard_with_limit("a*", 99), ["a", "ab", "abc"]);
}

#[test]
fn wildcard_matching_is_anchored_and_empty_patterns_are_not_match_all() {
    let trie: Trie = ["", "cat", "scatter", "🦀", "é"].into_iter().collect();
    assert_eq!(trie.wildcard("cat"), ["cat"]);
    assert_eq!(trie.wildcard("*cat*"), ["cat", "scatter"]);
    assert_eq!(trie.wildcard("?"), ["é", "🦀"]);
    assert_eq!(trie.wildcard(""), [""]);
    let mut folded = Trie::case_insensitive();
    folded.insert("Éclair");
    assert_eq!(folded.wildcard("É*"), ["éclair"]);
}
