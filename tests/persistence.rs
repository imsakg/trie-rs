use trie::Trie;

#[test]
fn writer_emits_a_versioned_sorted_dictionary() {
    let trie: Trie = ["pear", "apple", ""].into_iter().collect();
    let mut bytes = Vec::new();
    trie.to_writer(&mut bytes).unwrap();
    assert_eq!(
        String::from_utf8(bytes).unwrap(),
        "TRIE\t1\tS\n\napple\npear\n"
    );
    let mut folded = Trie::case_insensitive();
    folded.insert("PEAR");
    let mut bytes = Vec::new();
    folded.to_writer(&mut bytes).unwrap();
    assert_eq!(bytes, b"TRIE\t1\tI\npear\n");
}
