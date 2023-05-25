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
    load: Option<String>,
    save: Option<String>,
}

fn parse(args: Vec<String>) -> io::Result<(Options, Vec<String>)> {
    let mut options = Options {
        words: Vec::new(),
        limit: None,
        ignore_case: false,
        load: None,
        save: None,
    };
    let mut args = args.into_iter().peekable();
    loop {
        match args.peek().map(String::as_str) {
            Some("--ignore-case" | "-i") => {
                args.next();
                options.ignore_case = true;
            }
            Some("--save") => {
                args.next();
                options.save = Some(
                    args.next()
                        .ok_or_else(|| invalid("--save requires a path"))?,
                );
            }
            Some("--load") => {
                args.next();
                options.load = Some(
                    args.next()
                        .ok_or_else(|| invalid("--load requires a path"))?,
                );
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
    let mut trie = options
        .load
        .as_ref()
        .map(Trie::load)
        .transpose()?
        .unwrap_or_default();
    if options.ignore_case && !trie.is_case_insensitive() {
        let mut lowercase = Trie::case_insensitive();
        lowercase.extend(trie.words());
        trie = lowercase;
    }
    for path in options.words {
        trie.import_words(BufReader::new(File::open(path)?))?;
    }
    let code = execute(
        &mut trie,
        &args[0],
        &args[1..],
        options.limit.unwrap_or(usize::MAX),
        output,
    )?;
    if let Some(path) = options.save {
        trie.save(path)?;
    }
    Ok(code)
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
        "insert" => {
            if args.is_empty() {
                return Err(invalid("insert requires at least one word"));
            }
            let added = args.iter().filter(|word| trie.insert(word)).count();
            writeln!(output, "{added}")?;
            Ok(0)
        }
        "delete" => {
            if args.is_empty() {
                return Err(invalid("delete requires at least one word"));
            }
            let removed = args.iter().filter(|word| trie.remove(word)).count();
            writeln!(output, "{removed}")?;
            Ok(0)
        }
        "longest" => {
            arity(args, 1, command)?;
            match trie.longest_prefix(&args[0]) {
                Some(word) => {
                    writeln!(output, "{word}")?;
                    Ok(0)
                }
                None => Ok(1),
            }
        }
        "len" => {
            arity(args, 0, command)?;
            writeln!(output, "{}", trie.len())?;
            Ok(0)
        }
        "clear" => {
            arity(args, 0, command)?;
            let removed = trie.len();
            trie.clear();
            writeln!(output, "{removed}")?;
            Ok(0)
        }
        _ => Err(invalid(format!("unknown command: {command}"))),
    }
}
