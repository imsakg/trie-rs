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
    limit: Option<usize>,
    ignore_case: bool,
}

fn parse(args: Vec<String>) -> io::Result<(Options, Vec<String>)> {
    let mut options = Options {
        words: Vec::new(),
        limit: None,
        ignore_case: false,
    };
    let mut args = args.into_iter().peekable();
    loop {
        match args.peek().map(String::as_str) {
            Some("--ignore-case" | "-i") => {
                args.next();
                options.ignore_case = true;
            }
            Some("--limit") => {
                args.next();
                let value = args
                    .next()
                    .ok_or_else(|| invalid("--limit requires a number"))?;
                options.limit = Some(
                    value
                        .parse()
                        .map_err(|_| invalid("--limit requires a non-negative integer"))?,
                );
            }
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
    let mut trie = if options.ignore_case {
        Trie::case_insensitive()
    } else {
        Trie::new()
    };
    for path in options.words {
        trie.import_words(BufReader::new(File::open(path)?))?;
    }
    execute(
        &mut trie,
        &args[0],
        &args[1..],
        options.limit.unwrap_or(usize::MAX),
        output,
    )
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
    limit: usize,
    output: &mut impl Write,
) -> io::Result<u8> {
    match command {
        "list" => {
            arity(args, 0, command)?;
            trie.export_words(output)?;
            Ok(0)
        }
        "search" => {
            arity(args, 1, command)?;
            let found = trie.contains(&args[0]);
            writeln!(output, "{found}")?;
            Ok(u8::from(!found))
        }
        "prefix" => {
            arity(args, 1, command)?;
            let found = trie.has_prefix(&args[0]);
            writeln!(output, "{found}")?;
            Ok(u8::from(!found))
        }
        "count" => {
            arity(args, 1, command)?;
            writeln!(output, "{}", trie.count_prefix(&args[0]))?;
            Ok(0)
        }
        "complete" => {
            arity(args, 1, command)?;
            for word in trie.autocomplete_with_limit(&args[0], limit) {
                writeln!(output, "{word}")?;
            }
            Ok(0)
        }
        "match" => {
            arity(args, 1, command)?;
            for word in trie.wildcard_with_limit(&args[0], limit) {
                writeln!(output, "{word}")?;
            }
            Ok(0)
        }
        _ => Err(invalid(format!("unknown command: {command}"))),
    }
}
