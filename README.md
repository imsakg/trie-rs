# trie-rs

A deterministic character trie and dictionary CLI for UTF-8 strings. The Cargo
package and Rust crate are named `trie`; the repository is `trie-rs`. Rust 2021,
standard library only, no third-party dependencies.

## Library

```rust
use trie::Trie;

let mut words: Trie = ["car", "cart", "cat"].into_iter().collect();
assert!(words.contains("car"));
assert!(!words.contains("ca"));
assert!(words.has_prefix("ca"));
assert_eq!(words.autocomplete_with_limit("ca", 2), ["car", "cart"]);
assert_eq!(words.wildcard("c?t"), ["cat"]);
assert!(words.remove("car"));
assert!(words.contains("cart"));
```

- Owned and borrowed strings; duplicate insertions have set semantics.
- Insertion/deletion return whether membership changed; deletion prunes unused paths.
- Exact/prefix search, sorted autocomplete with limits, prefix counts, longest prefix.
- Whole-word `?`/`*` wildcard matching with backslash escapes.
- Opt-in `Trie::case_insensitive()` using Unicode lowercase keys, not full case folding.
- Empty strings, punctuation, spaces, non-ASCII keys, and escaped control characters.
- Versioned reader/writer/file persistence, safe sibling-file replacement, plain word-list import/export.
- `Extend`, `FromIterator`, independent `Clone`, constant-time `len`, and `clear`.
- Iterative tree operations and regression coverage for 100,000-scalar paths.

## CLI

```sh
cargo run -- --help
cargo run -- --words data/words.txt --limit 3 complete ca
cargo run -- -i --save dictionary.trie insert Rust Ruby RUST
cargo run -- --load dictionary.trie search RUST
cargo run -- --load dictionary.trie match 'r?st'
cargo run -- --load dictionary.trie --save dictionary.trie delete ruby
```

Global options precede the command. Mutations are in-memory unless `--save` is
explicit. `search`, `prefix`, and `longest` exit 1 for no match; usage and I/O
errors exit 2. Other successful commands exit 0, including an empty completion.
Use `cargo install --path .` to install the local `trie` binary.

## Documentation and checks

- [CLI guide](docs/cli.md): complete syntax, flags, exit codes, and persistence behavior.
- [Query semantics](docs/query-semantics.md): Unicode normalization, ordering, wildcards.
- [Snapshot format](docs/file-format.md): encoding, strict validation, write guarantees.
- [Complexity](docs/complexity.md): invariants, algorithm costs, resource limitations.
- [Contributing](CONTRIBUTING.md): quality gates and test strategy.
- `cargo doc --no-deps --open` for generated API documentation.
- `cargo run --example dictionary` for an executable library example.
- `bash scripts/smoke-cli.sh` for an isolated end-to-end CLI check.

Run `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
`cargo test --all-targets --all-features`, and `cargo test --doc` before changing
behavior. CI is configured for Linux, macOS, and Windows; remote execution is
separate from a local test pass.

[HISTORY.md](HISTORY.md) documents deliberately reconstructed commit timestamps.
This expansion is local and has not been pushed. `lorem.txt` is retained as the
original demo's historical input; no command implicitly reads it.
