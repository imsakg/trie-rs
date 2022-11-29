//! Versioned UTF-8 persistence; one escaped word per line.
use crate::Trie;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

impl Trie {
    /// Export sorted LF-terminated words. Reject CR/LF-containing words before writing.
    pub fn export_words<W: Write>(&self, mut writer: W) -> io::Result<()> {
        let words = self.words();
        if words.iter().any(|word| word.contains(['\r', '\n'])) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "word list cannot represent CR or LF inside a word",
            ));
        }
        for word in words {
            writeln!(writer, "{word}")?;
        }
        writer.flush()
    }

    /// Import one UTF-8 word per line, preserving spaces and counting new keys.
    /// LF/CRLF are accepted. On an I/O error, previously read words remain inserted.
    pub fn import_words<R: BufRead>(&mut self, reader: R) -> io::Result<usize> {
        let mut added = 0;
        for line in reader.lines() {
            added += usize::from(self.insert(line?));
        }
        Ok(added)
    }

    /// Save via a synced sibling temporary file and rename it over the destination.
    /// Failure before rename leaves an existing destination untouched.
    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        if path.file_name().is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "destination must name a file",
            ));
        }
        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let (mut file, mut pending) = temporary_file(parent, path)?;
        self.to_writer(&mut file)?;
        file.sync_all()?;
        drop(file);
        std::fs::rename(pending.0.as_ref().expect("temporary file is armed"), path)?;
        pending.0 = None;
        Ok(())
    }

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

struct PendingFile(Option<std::path::PathBuf>);
impl Drop for PendingFile {
    fn drop(&mut self) {
        if let Some(path) = &self.0 {
            let _ = std::fs::remove_file(path);
        }
    }
}

fn temporary_file(parent: &Path, destination: &Path) -> io::Result<(File, PendingFile)> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    for _ in 0..32 {
        let path = parent.join(format!(
            ".trie-{}-{}.tmp",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        if path.file_name() == destination.file_name() {
            continue;
        }
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(file) => return Ok((file, PendingFile(Some(path)))),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "cannot reserve snapshot temporary file",
    ))
}
