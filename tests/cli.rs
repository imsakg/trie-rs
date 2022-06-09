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
