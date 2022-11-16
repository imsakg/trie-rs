use trie::Trie;
mod common;

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

#[test]
fn reader_restores_words_and_case_configuration() {
    let restored = Trie::from_reader(&b"TRIE\t1\tI\n\napple\nPEAR\n"[..]).unwrap();
    assert!(restored.is_case_insensitive());
    assert_eq!(restored.words(), ["", "apple", "pear"]);
    assert!(restored.contains("APPLE"));
    let empty = Trie::from_reader(&b"TRIE\t1\tS\n"[..]).unwrap();
    assert!(empty.is_empty());
    assert!(!empty.is_case_insensitive());
}

#[test]
fn persistence_round_trips_every_special_scalar_and_empty_word() {
    let trie: Trie = [
        "",
        "a\nb",
        "a\rb",
        "a\tb",
        "nul\0byte",
        "slash\\\\",
        "É🦀",
        " ",
    ]
    .into_iter()
    .collect();
    let mut bytes = Vec::new();
    trie.to_writer(&mut bytes).unwrap();
    let restored = Trie::from_reader(bytes.as_slice()).unwrap();
    assert_eq!(restored.words(), trie.words());
    let mut second = Vec::new();
    restored.to_writer(&mut second).unwrap();
    assert_eq!(bytes, second);
}

#[test]
fn corrupt_snapshots_fail_instead_of_silently_losing_data() {
    use std::io::ErrorKind;
    for bytes in [
        &b""[..],
        &b"TRIE\t9\tS\n"[..],
        &b"TRIE\t1\tX\n"[..],
        &b"TRIE\t1\tS"[..],
        &b"TRIE\t1\tS\nword"[..],
        &b"TRIE\t1\tS\nbad\\q\n"[..],
        &b"TRIE\t1\tS\n\\\n"[..],
        &b"TRIE\t1\tS\n\xff\n"[..],
    ] {
        assert_eq!(
            Trie::from_reader(bytes).unwrap_err().kind(),
            ErrorKind::InvalidData,
            "{bytes:?}"
        );
    }
}

#[test]
fn load_reads_a_dictionary_file_and_reports_missing_paths() {
    let temp = common::temp_dir();
    let path = temp.path().join("words.trie");
    std::fs::write(&path, "TRIE\t1\tS\nhello\n").unwrap();
    assert_eq!(Trie::load(&path).unwrap().words(), ["hello"]);
    assert_eq!(
        Trie::load(temp.path().join("missing")).unwrap_err().kind(),
        std::io::ErrorKind::NotFound
    );
}
