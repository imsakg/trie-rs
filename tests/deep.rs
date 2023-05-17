use std::process::Command;
use trie::Trie;

fn isolated(name: &str, operation: impl FnOnce()) {
    if std::env::var_os("TRIE_DEEP_CHILD").is_some() {
        operation();
        return;
    }
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name])
        .env("TRIE_DEEP_CHILD", "1")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "child failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn dropping_and_clearing_deep_paths_are_stack_safe() {
    isolated("dropping_and_clearing_deep_paths_are_stack_safe", || {
        let mut trie = Trie::new();
        trie.insert("a".repeat(100_000));
        drop(trie);
        let mut trie = Trie::new();
        trie.insert("b".repeat(100_000));
        trie.clear();
        assert!(trie.is_empty());
    });
}
