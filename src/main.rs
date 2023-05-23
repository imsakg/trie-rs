use lib::Trie;
use std::{fs::File, io::BufRead};
mod lib;

const TEST: [&str; 10] = [
    "hello",
    "world",
    "foo",
    "bar",
    "baz",
    "fooloma",
    "respectable",
    "lorem",
    "ipsum",
    "dolor",
];

fn main() {
    let mut trie = Trie::new();

    let file = File::open("lorem.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            if word
                .chars()
                .any(|c| c.is_ascii_punctuation() || c.is_ascii_digit() || c.is_uppercase())
            {
                continue;
            }
            trie.insert(word.to_string());
        }
        if trie.contains("respectable".to_string()) {
            break;
        }
    }

    // for word in TEST.iter() {
    //     trie.insert(word.to_string());
    // }

    for word in TEST.iter() {
        println!("{}: {}", word, trie.contains(word.to_string()));
    }

    trie.print();
}
