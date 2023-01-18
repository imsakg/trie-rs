# CLI guide

Build with `cargo build --release`, run with `cargo run -- ...`, or install the
local binary with `cargo install --path .`.

```text
trie [OPTIONS] COMMAND [ARGS]
```

## Global options

Options must appear **before** the command. After it, values are literal command
arguments, so `trie insert --help` inserts the key `--help` rather than showing help.
`trie --help` and `trie --version` work alone; no arguments prints help.

- `--words PATH`: import a UTF-8 word list. Repeat to merge sources in order.
- `--load PATH`: load a versioned snapshot first, including its case configuration.
- `--save PATH`: save the resulting dictionary after a successful command.
- `-i`, `--ignore-case`: normalize keys to Unicode lowercase. On a sensitive loaded
  snapshot this explicitly converts keys and may collapse duplicates.
- `--limit N`: first N lexical results, for `complete`/`match` only. Zero is valid.
- `--`: stop global-option parsing. A command must still follow.

Repeated scalar options (`--load`, `--save`, `--limit`) use the last supplied value.
All arguments, including paths, must be valid UTF-8; invalid arguments fail without
panicking. Word lists accept LF or CRLF, preserve spaces, and interpret blank lines
as the empty word. Snapshot files are a different, strict format.

## Commands and stdout

- `insert WORD...`: print count of newly inserted distinct normalized words.
- `delete WORD...`: print count of removed words; missing words are harmless.
- `search WORD`: print `true`/`false` for complete-word membership.
- `prefix PREFIX`: print `true`/`false` for a prefix path.
- `complete PREFIX`: print sorted completions, optionally bounded by `--limit`.
- `match PATTERN`: print sorted whole-word glob matches. Shell-quote patterns.
- `count PREFIX`: print the number of stored words below a prefix.
- `longest INPUT`: print the longest stored prefix, or nothing if none exists.
- `list`: print all keys as a sorted plain word list.
- `len`: print distinct-word count.
- `clear`: remove all words and print removed count, retaining case configuration.
- `import PATH`: import a word list and print count of newly inserted words.
- `export PATH`: write a sorted plain word list to a file, no stdout payload.
- `tree`: print a plain edge tree with terminal markers, no ANSI color.

Results are LF-terminated, and boolean/count commands have one line. Query output
prints literal keys; embedded LF/CR keys are not unambiguously line-oriented.
`list` and `export` reject such keys; use versioned snapshots for lossless storage.
The diagnostic tree is intended for small, readable dictionaries.

## Exit status and file effects

- **0:** command completed; empty completion/glob results are not errors.
- **1:** `search`, `prefix`, or `longest` found no match.
- **2:** invalid options/arity, missing files, corrupt snapshot, write failure.

Errors go to stderr prefixed by `trie:`. Loading never edits the source. Mutation
is ephemeral without `--save`; to edit a snapshot, pass the same path to `--load`
and `--save`. An explicit save also runs after a no-match query (exit 1).
Snapshot saving uses a sibling temporary file and atomic rename, with documented
[limits](file-format.md). `export` validates representability before opening its
destination, but plain-list export is a direct file write, not atomic replacement.
There is no interactive prompt or implicit save location.

## Example workflow

```sh
trie -i --words data/words.txt --save dictionary.trie len
trie --load dictionary.trie complete ru
trie --load dictionary.trie --limit 3 match '*a*'
trie --load dictionary.trie --save dictionary.trie insert rustacean
trie --load dictionary.trie --save dictionary.trie delete ruby
trie --load dictionary.trie export words.txt
```

For a self-cleaning workflow, run `bash scripts/smoke-cli.sh` from the checkout.
