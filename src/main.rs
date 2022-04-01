use std::{
    io::{self, Write},
    process::ExitCode,
};

const HELP: &str = "Usage: trie [OPTIONS] COMMAND [ARGS]\n\nA UTF-8 dictionary trie.\n\nOptions:\n  -h, --help  Show help\n";

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

fn run() -> io::Result<u8> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args == ["--help"] || args == ["-h"] {
        io::stdout().lock().write_all(HELP.as_bytes())?;
        return Ok(0);
    }
    Err(invalid(format!("unknown command: {}", args[0])))
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
