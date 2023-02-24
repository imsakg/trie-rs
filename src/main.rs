mod cli;

use std::{
    io::{self, Write},
    process::ExitCode,
};

const HELP: &str = "Usage: trie [OPTIONS] COMMAND [ARGS]\n\nA UTF-8 dictionary trie. Global options must precede COMMAND.\n\nOptions:\n  --words PATH       Import a UTF-8 word list (repeatable)\n  --load PATH        Load a versioned snapshot\n  --save PATH        Save the resulting dictionary atomically\n  -i, --ignore-case  Store and query Unicode lowercase keys\n  --limit N          Limit complete/match results (zero is allowed)\n  -h, --help         Show help\n  -V, --version      Print package version\n\nCommands:\n  insert WORD...     Insert words; print new-word count\n  delete WORD...     Remove words; print removed-word count\n  search WORD        Print true/false for a complete word\n  prefix PREFIX      Print true/false for an edge path\n  complete PREFIX    Print sorted completions\n  match PATTERN      Print sorted matches (? * and backslash escapes)\n  count PREFIX       Print number of prefix matches\n  longest INPUT      Print longest stored prefix\n  list               Print a sorted plain word list\n  len                Print distinct-word count\n  clear              Remove all words; print removed-word count\n  import PATH        Import words; print new-word count\n  export PATH        Write a sorted plain word list\n  tree               Print the plain-text edge tree\n\nExit codes: 0 success; 1 search/prefix/longest found no match; 2 usage or I/O error.\nMutations are in-memory unless --save is specified. Patterns should be shell-quoted.\n";

fn run() -> io::Result<u8> {
    let args: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|arg| {
            arg.into_string().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidInput, "arguments must be valid UTF-8")
            })
        })
        .collect::<io::Result<_>>()?;
    if args == ["--version"] || args == ["-V"] {
        writeln!(io::stdout().lock(), "trie {}", env!("CARGO_PKG_VERSION"))?;
        return Ok(0);
    }
    if args.is_empty() || args == ["--help"] || args == ["-h"] {
        io::stdout().lock().write_all(HELP.as_bytes())?;
        return Ok(0);
    }
    cli::run(args, &mut io::stdout().lock())
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("trie: {error}");
            ExitCode::from(2)
        }
    }
}
