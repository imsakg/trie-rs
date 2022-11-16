//! Versioned UTF-8 persistence; one escaped word per line.
use crate::Trie;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

impl Trie {
    /// Load a versioned snapshot from a UTF-8 dictionary file.
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        Self::from_reader(BufReader::new(File::open(path)?))
    }

    /// Read a v1 snapshot; invalid UTF-8, headers, escapes, and unterminated lines fail.
    pub fn from_reader<R: BufRead>(mut reader: R) -> io::Result<Self> {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let mut trie = match line.as_str() {
            "TRIE\t1\tS\n" => Self::new(),
            "TRIE\t1\tI\n" => Self::case_insensitive(),
            _ => return Err(invalid("unsupported or missing dictionary header")),
        };
        loop {
            line.clear();
            if reader.read_line(&mut line)? == 0 {
                break;
            }
            let Some(word) = line.strip_suffix('\n') else {
                return Err(invalid("unterminated word record"));
            };
            trie.insert(decode(word)?);
        }
        Ok(trie)
    }

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

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn decode(word: &str) -> io::Result<String> {
    let mut output = String::new();
    let mut chars = word.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }
        output.push(match chars.next() {
            Some('\\') => '\\',
            Some('n') => '\n',
            Some('r') => '\r',
            Some('t') => '\t',
            Some('0') => '\0',
            _ => return Err(invalid("invalid word escape")),
        });
    }
    Ok(output)
}
