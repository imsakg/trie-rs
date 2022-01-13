use trie::Trie;

fn main() -> std::io::Result<()> {
    let mut trie = Trie::case_insensitive();
    trie.extend(["Rust", "Ruby", "RUST", "Python"]);
    assert_eq!(trie.autocomplete_with_limit("RU", 2), ["ruby", "rust"]);
    assert_eq!(trie.wildcard("r?st"), ["rust"]);
    assert!(trie.remove("RUBY"));
    let mut bytes = Vec::new();
    trie.to_writer(&mut bytes)?;
    let restored = Trie::from_reader(bytes.as_slice())?;
    assert_eq!(restored.words(), ["python", "rust"]);
    println!("{}", restored.words().join(", "));
    Ok(())
}
