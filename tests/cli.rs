use std::process::{Command, Output};
mod common;

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_trie"))
        .args(args)
        .output()
        .unwrap()
}
fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}

#[test]
fn help_is_explicit_and_does_not_read_lorem_from_the_working_directory() {
    let output = cli(&["--help"]);
    assert!(output.status.success());
    assert!(stdout(&output).contains("Usage: trie"));
    assert!(output.stderr.is_empty());
}

#[test]
fn list_imports_all_words_instead_of_filtering_punctuation_and_case() {
    let temp = common::temp_dir();
    let path = temp.path().join("words.txt");
    std::fs::write(&path, "zebra\nApple\na-b\n42\nzebra\n").unwrap();
    let output = cli(&["--words", path.to_str().unwrap(), "list"]);
    assert!(output.status.success(), "{:?}", output);
    assert_eq!(stdout(&output), "42\nApple\na-b\nzebra\n");
}

#[test]
fn search_prints_membership_and_sets_a_no_match_exit_status() {
    let temp = common::temp_dir();
    let path = temp.path().join("words");
    std::fs::write(&path, "rust\n").unwrap();
    let path = path.to_str().unwrap();
    let found = cli(&["--words", path, "search", "rust"]);
    assert_eq!(found.status.code(), Some(0));
    assert_eq!(stdout(&found), "true\n");
    let absent = cli(&["--words", path, "search", "rus"]);
    assert_eq!(absent.status.code(), Some(1));
    assert_eq!(stdout(&absent), "false\n");
}

#[test]
fn prefix_and_count_commands_query_paths_without_requiring_a_word() {
    let temp = common::temp_dir();
    let path = temp.path().join("words");
    std::fs::write(&path, "car\ncart\ncat\n").unwrap();
    let path = path.to_str().unwrap();
    let prefix = cli(&["--words", path, "prefix", "ca"]);
    assert_eq!(prefix.status.code(), Some(0));
    assert_eq!(stdout(&prefix), "true\n");
    let count = cli(&["--words", path, "count", "ca"]);
    assert!(count.status.success());
    assert_eq!(stdout(&count), "3\n");
    assert_eq!(
        cli(&["--words", path, "prefix", "z"]).status.code(),
        Some(1)
    );
}

#[test]
fn complete_honors_a_global_numeric_limit() {
    let temp = common::temp_dir();
    let path = temp.path().join("words");
    std::fs::write(&path, "cat\ncart\ncar\ndog\n").unwrap();
    let path = path.to_str().unwrap();
    let output = cli(&["--words", path, "--limit", "2", "complete", "ca"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "car\ncart\n");
    assert_eq!(
        stdout(&cli(&["--words", path, "--limit", "0", "complete", "ca"])),
        ""
    );
}

#[test]
fn match_command_supports_globs_without_shell_expansion() {
    let temp = common::temp_dir();
    let path = temp.path().join("words");
    std::fs::write(&path, "cat\ncot\ncart\ndog\n").unwrap();
    let path = path.to_str().unwrap();
    let output = cli(&["--words", path, "match", "c?t"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "cat\ncot\n");
    assert_eq!(
        stdout(&cli(&["--words", path, "--limit", "1", "match", "c*"])),
        "cart\n"
    );
}

#[test]
fn ignore_case_normalizes_loaded_word_lists_and_queries() {
    let temp = common::temp_dir();
    let path = temp.path().join("words");
    std::fs::write(&path, "Rust\nRUST\nRuby\n").unwrap();
    let path = path.to_str().unwrap();
    let output = cli(&["--ignore-case", "--words", path, "complete", "RU"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "ruby\nrust\n");
    assert_eq!(
        cli(&["-i", "--words", path, "search", "rUsT"])
            .status
            .code(),
        Some(0)
    );
    assert_eq!(
        cli(&["--words", path, "search", "rust"]).status.code(),
        Some(1)
    );
}

#[test]
fn snapshot_loading_preserves_mode_and_can_explicitly_lowercase_keys() {
    let temp = common::temp_dir();
    let path = temp.path().join("words.trie");
    let mut folded = trie::Trie::case_insensitive();
    folded.insert("Rust");
    folded.save(&path).unwrap();
    let output = cli(&["--load", path.to_str().unwrap(), "search", "RUST"]);
    assert_eq!(output.status.code(), Some(0));
    let sensitive: trie::Trie = ["Rust", "RUST"].into_iter().collect();
    sensitive.save(&path).unwrap();
    let output = cli(&["--load", path.to_str().unwrap(), "-i", "list"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "rust\n");
}

#[test]
fn insert_counts_new_words_and_persists_them_when_save_is_requested() {
    let temp = common::temp_dir();
    let path = temp.path().join("dictionary.trie");
    let output = cli(&[
        "-i",
        "--save",
        path.to_str().unwrap(),
        "insert",
        "Rust",
        "RUST",
        "Ruby",
        "",
    ]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "3\n");
    let restored = trie::Trie::load(&path).unwrap();
    assert!(restored.is_case_insensitive());
    assert_eq!(restored.words(), ["", "ruby", "rust"]);
}

#[test]
fn delete_prunes_only_requested_words_and_can_replace_the_loaded_snapshot() {
    let temp = common::temp_dir();
    let path = temp.path().join("dictionary.trie");
    let trie: trie::Trie = ["car", "cart", "cat"].into_iter().collect();
    trie.save(&path).unwrap();
    let output = cli(&[
        "--load",
        path.to_str().unwrap(),
        "--save",
        path.to_str().unwrap(),
        "delete",
        "car",
        "missing",
    ]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "1\n");
    let restored = trie::Trie::load(&path).unwrap();
    assert_eq!(restored.words(), ["cart", "cat"]);
    assert!(restored.has_prefix("car"));
}

#[test]
fn longest_prints_the_longest_stored_prefix_or_exits_one() {
    let temp = common::temp_dir();
    let path = temp.path().join("words");
    std::fs::write(&path, "a\napp\napple\n").unwrap();
    let path = path.to_str().unwrap();
    let output = cli(&["--words", path, "longest", "applesauce"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "apple\n");
    let absent = cli(&["--words", path, "longest", "z"]);
    assert_eq!(absent.status.code(), Some(1));
    assert!(absent.stdout.is_empty());
}

#[test]
fn len_and_clear_commands_report_distinct_and_removed_word_counts() {
    let temp = common::temp_dir();
    let path = temp.path().join("words");
    let saved = temp.path().join("empty.trie");
    std::fs::write(&path, "a\na\nb\n").unwrap();
    let path = path.to_str().unwrap();
    assert_eq!(stdout(&cli(&["--words", path, "len"])), "2\n");
    let output = cli(&[
        "-i",
        "--words",
        path,
        "--save",
        saved.to_str().unwrap(),
        "clear",
    ]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "2\n");
    let restored = trie::Trie::load(&saved).unwrap();
    assert!(restored.is_empty());
    assert!(restored.is_case_insensitive());
}

#[test]
fn import_and_export_commands_interchange_plain_word_lists() {
    let temp = common::temp_dir();
    let source = temp.path().join("source");
    let snapshot = temp.path().join("snapshot");
    let destination = temp.path().join("exported");
    std::fs::write(&source, "Pear\napple\nPear\n").unwrap();
    let output = cli(&[
        "-i",
        "--save",
        snapshot.to_str().unwrap(),
        "import",
        source.to_str().unwrap(),
    ]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "2\n");
    let output = cli(&[
        "--load",
        snapshot.to_str().unwrap(),
        "export",
        destination.to_str().unwrap(),
    ]);
    assert!(output.status.success());
    assert_eq!(std::fs::read(destination).unwrap(), b"apple\npear\n");
}

#[test]
fn tree_command_prints_stable_plain_text() {
    let temp = common::temp_dir();
    let path = temp.path().join("words");
    std::fs::write(&path, "b\nab\na\n").unwrap();
    let output = cli(&["--words", path.to_str().unwrap(), "tree"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "a (*)\n b (*)\nb (*)\n");
    assert!(!output.stdout.contains(&27));
}

#[test]
fn version_and_no_argument_help_are_safe_outside_the_repository() {
    let output = cli(&["--version"]);
    assert!(output.status.success());
    assert_eq!(
        stdout(&output),
        format!("trie {}\n", env!("CARGO_PKG_VERSION"))
    );
    let temp = common::temp_dir();
    let output = Command::new(env!("CARGO_BIN_EXE_trie"))
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(stdout(&output).contains("Usage: trie"));
}

#[cfg(unix)]
#[test]
fn non_utf8_arguments_report_an_error_instead_of_panicking() {
    use std::os::unix::ffi::OsStringExt;
    let output = Command::new(env!("CARGO_BIN_EXE_trie"))
        .arg(std::ffi::OsString::from_vec(vec![255]))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("UTF-8"));
    assert!(!String::from_utf8_lossy(&output.stderr).contains("panicked"));
}

#[test]
fn invalid_limits_and_argument_counts_never_save_a_dictionary() {
    let temp = common::temp_dir();
    let path = temp.path().join("must-not-exist");
    let path = path.to_str().unwrap();
    for args in [
        vec!["--save", path, "--limit", "-1", "complete", ""],
        vec!["--save", path, "--limit", "x", "match", "*"],
        vec!["--save", path, "--limit", "1", "len"],
        vec!["--save", path, "search"],
        vec!["--save", path, "insert"],
        vec!["--save", path, "delete"],
        vec!["--save", path, "list", "extra"],
        vec!["--save", path, "unknown"],
    ] {
        let output = cli(&args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(!output.stderr.is_empty());
        assert!(!std::path::Path::new(path).exists());
    }
}

#[test]
fn missing_options_unknown_flags_and_unreadable_files_have_visible_errors() {
    for args in [
        vec!["--words"],
        vec!["--load"],
        vec!["--save"],
        vec!["--limit"],
        vec!["--mystery"],
        vec!["--"],
    ] {
        let output = cli(&args);
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).starts_with("trie:"));
    }
    let temp = common::temp_dir();
    let missing = temp.path().join("missing");
    let corrupt = temp.path().join("corrupt");
    std::fs::write(&corrupt, "not a snapshot").unwrap();
    for option in ["--words", "--load"] {
        let output = cli(&[option, missing.to_str().unwrap(), "list"]);
        assert_eq!(output.status.code(), Some(2));
        assert!(!output.stderr.is_empty());
    }
    assert_eq!(
        cli(&["--load", corrupt.to_str().unwrap(), "list"])
            .status
            .code(),
        Some(2)
    );
}

#[test]
fn empty_result_lists_are_successful_but_failed_boolean_queries_exit_one() {
    assert_eq!(cli(&["search", "missing"]).status.code(), Some(1));
    assert_eq!(cli(&["prefix", "missing"]).status.code(), Some(1));
    for args in [["complete", "missing"], ["match", "*"]] {
        let output = cli(&args);
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn repeated_word_sources_merge_and_unsaved_mutations_leave_the_snapshot_untouched() {
    let temp = common::temp_dir();
    let a = temp.path().join("a");
    let b = temp.path().join("b");
    let snapshot = temp.path().join("snapshot");
    std::fs::write(&a, "a\nb\n").unwrap();
    std::fs::write(&b, "b\nc\n").unwrap();
    let output = cli(&[
        "--words",
        a.to_str().unwrap(),
        "--words",
        b.to_str().unwrap(),
        "--save",
        snapshot.to_str().unwrap(),
        "len",
    ]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "3\n");
    let original = std::fs::read(&snapshot).unwrap();
    let output = cli(&["--load", snapshot.to_str().unwrap(), "delete", "a", "b"]);
    assert!(output.status.success());
    assert_eq!(stdout(&output), "2\n");
    assert_eq!(std::fs::read(&snapshot).unwrap(), original);
}
