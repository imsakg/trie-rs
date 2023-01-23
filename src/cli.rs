use std::{
    fs::File,
    io::{self, BufReader, Write},
};
use trie::Trie;

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

struct Options {
    words: Vec<String>,
}

fn parse(args: Vec<String>) -> io::Result<(Options, Vec<String>)> {
    let mut options = Options { words: Vec::new() };
    let mut args = args.into_iter().peekable();
    loop {
        match args.peek().map(String::as_str) {
            Some("--words") => {
                args.next();
                options.words.push(
                    args.next()
                        .ok_or_else(|| invalid("--words requires a path"))?,
                );
            }
            Some("--") => {
                args.next();
                break;
            }
            Some(flag) if flag.starts_with('-') => {
                return Err(invalid(format!("unknown option: {flag}")))
            }
            _ => break,
        }
    }
    let rest: Vec<_> = args.collect();
    if rest.is_empty() {
        return Err(invalid("a command is required"));
    }
    Ok((options, rest))
}

pub fn run(args: Vec<String>, output: &mut impl Write) -> io::Result<u8> {
    let (options, args) = parse(args)?;
    let mut trie = Trie::new();
    for path in options.words {
        trie.import_words(BufReader::new(File::open(path)?))?;
    }
    execute(&mut trie, &args[0], &args[1..], output)
}

fn arity(args: &[String], expected: usize, command: &str) -> io::Result<()> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(invalid(format!("{command} expects {expected} argument(s)")))
    }
}

fn execute(
    trie: &mut Trie,
    command: &str,
    args: &[String],
    output: &mut impl Write,
) -> io::Result<u8> {
    match command {
        "list" => {
            arity(args, 0, command)?;
            trie.export_words(output)?;
            Ok(0)
        }
        _ => Err(invalid(format!("unknown command: {command}"))),
    }
}
