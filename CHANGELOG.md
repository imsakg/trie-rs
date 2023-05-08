# Changelog

## Unreleased local expansion

### Added

- Borrowed-string insertion and exact lookup, unique-word counts, pruning deletion,
  clear, iterator-based construction/extension, independent cloning.
- Deterministic prefix search, sorted bounded autocomplete, prefix counts, longest
  stored-prefix lookup, Unicode-scalar glob matching with `?`, `*`, and escapes.
- Opt-in Unicode-lowercase dictionaries, consistently used by reads, writes,
  deletions, prefix queries, wildcards, and persistence.
- Versioned escaped UTF-8 snapshots, strict parsing, buffered load, sibling-file
  atomic save, and plain word-list import/export.
- A noninteractive CLI with explicit loading/saving, useful exit codes, visible
  errors, limits, full help, and a version command.
- Model/differential tests, isolated deep-path regressions, CLI integration tests,
  runnable examples, documentation, and a cross-platform CI definition.

### Changed

- `insert` returns a membership-change boolean while retaining owned-string call
  compatibility. Existing code that explicitly requires a unit return must adapt.
- The binary no longer implicitly reads `lorem.txt`, discards punctuation/case,
  breaks on an arbitrary word, or prints a hardcoded test dictionary.
- `print` borrows rather than consumes the trie and no longer emits ANSI colors;
  `tree` provides a testable string representation.
- Debug output is a bounded summary, not a dump of every descendant.
- Removed the terminal-color dependency. There are no third-party dependencies.
- Long valid keys no longer recurse through removal, cloning, clear, drop, or
  partial-load cleanup. Tests cover 100,000-scalar paths.

### Provenance and scope

This work is local and not pushed. [HISTORY.md](HISTORY.md) explains intentionally
reconstructed author/committer dates; they are not evidence of historical work.
The original two commits and original input file remain in the ancestry/checkout.
No release is published by this change. Case folding, locale collation, fuzzy or
ranked search, input quotas, authenticated snapshots, and compressed radix nodes
remain outside scope. See the documented resource and write-durability limits.
