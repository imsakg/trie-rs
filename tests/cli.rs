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
