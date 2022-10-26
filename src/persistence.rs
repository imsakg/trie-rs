//! Versioned UTF-8 persistence; one escaped word per line.
use crate::Trie;
use std::io::{self, Write};

impl Trie {
    /// Write a deterministic v1 dictionary and flush the writer.
    pub fn to_writer<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writeln!(
            writer,
            "TRIE\t1\t{}",
            if self.is_case_insensitive() { "I" } else { "S" }
        )?;
        for word in self.words() {
            writeln!(writer, "{}", encode(&word))?;
        }
        writer.flush()
    }
}

fn encode(word: &str) -> String {
    let mut output = String::new();
    for ch in word.chars() {
        match ch {
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\0' => output.push_str("\\0"),
            other => output.push(other),
        }
    }
    output
}
