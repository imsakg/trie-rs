mod cli;

use std::{
    io::{self, Write},
    process::ExitCode,
};

const HELP: &str = "Usage: trie [OPTIONS] COMMAND [ARGS]\n\nA UTF-8 dictionary trie.\n\nCommands: list\nGlobal options must precede COMMAND.\n  --words PATH  Import a UTF-8 word list\n\nOptions:\n  -h, --help  Show help\n";

fn run() -> io::Result<u8> {
    let args: Vec<String> = std::env::args().skip(1).collect();
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
