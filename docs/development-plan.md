# Trie library and CLI implementation plan

> **For Hermes:** Execute focused, test-backed slices; independently review before handoff.

**Goal:** Turn the original character-trie demo into a deterministic dictionary library and a usable command-line tool.

**Architecture:** Keep the existing `trie` crate and `Trie` type. A character-edge tree stores set membership; ordered children provide reproducible traversal. Query helpers are independent of I/O; persistence uses a versioned, validated text format; the CLI is an application boundary.

**Tech stack:** Rust 2021, Cargo, standard-library data structures and I/O. Avoid new runtime dependencies unless needed.

## Sequence and gates

1. **Characterize and repair the baseline.** `tests/core.rs`, `src/main.rs`, `src/lib.rs`: lock in exact membership, then use the library target correctly. Run `cargo test`.
2. **Set operations.** `src/lib.rs`, `tests/core.rs`: borrowed strings, unique-word counts, prefix existence, pruning deletion, clear, collection traits. For each new API, write a failing integration test, implement the smallest behavior, and rerun the suite.
3. **Queries.** `src/lib.rs`, `tests/queries.rs`: sorted enumeration, prefix completion with limits, prefix counts, longest-prefix lookup, and `?`/`*` wildcard matching. Cover empty strings and Unicode scalar values explicitly.
4. **Normalization.** `src/lib.rs`, `tests/case.rs`: opt-in Unicode lowercase mode, applied consistently to writes, reads, deletion, and patterns. State that lowercase is not locale-aware Unicode case folding.
5. **Persistence.** `src/persistence.rs`, `tests/persistence.rs`: versioned escaped line format, strict corruption checks, reader/writer and file APIs, round-trip tests. Preserve options and all legal strings, including tabs, newlines, and the empty word.
6. **CLI.** `src/main.rs`, `tests/cli.rs`: explicit help, dictionary loading, search/prefix/complete/match, insert/delete and persistence, options, meaningful errors and exit codes. Spawn the actual Cargo-built binary in integration tests.
7. **Hardening and documentation.** `tests/model.rs`, `examples/`, `README.md`, `docs/`: deterministic model checks, regressions, runnable examples, complexity and file-format contracts.

## Acceptance

- All requested features work via real library and/or CLI calls, not placeholder APIs.
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets --all-features`, `cargo test --doc`, and `cargo doc --no-deps` pass.
- Every added commit changes actual source, tests, examples, or documentation; no empty count-padding commits.
- Preserve the original ancestry. Target 90 reachable commits after inspection found 2 original commits, not the estimated 11.
- No push or other remote mutation. Validate local count, nonempty diffs, cleanliness, and assigned date range after the final rebase.

## Deferred

Fuzzy edit distance, ranked usage statistics, compressed radix edges, locale-specific collation, and concurrent mutation are outside this change.
