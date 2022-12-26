# Query semantics

- `contains(word)` asks for a complete stored key, not an intermediate path.
- `has_prefix(prefix)` asks for a path. The empty prefix always exists.
- `autocomplete(prefix)` returns all complete keys below that path, including the
  prefix itself if it is a word. `autocomplete_with_limit(prefix, n)` stops after
  the first `n` sorted results. A zero limit returns nothing.
- `count_prefix(prefix)` counts stored words below a path without allocating
  strings. It returns zero for a missing path.
- `longest_prefix(input)` returns the longest *stored word* beginning the input,
  not the longest path. If the empty word is stored, it can be the answer.
- `words()` is equivalent to completing the empty prefix.

## Ordering and normalization

Results use Rust string lexicographic order (Unicode scalar/UTF-8 order), not
locale collation. Duplicates collapse. Insensitive dictionaries store Unicode
`to_lowercase()` keys and return those keys; original spellings are not retained.
Every operation applies the same lowercase rule. This is **not** full Unicode
case folding: `Straße` does not equal `STRASSE`, and Turkish dotted I may expand
to two scalars. No canonical Unicode normalization is performed.

## Wildcards

`wildcard(pattern)` matches the entire stored word. `?` consumes exactly one
Unicode scalar value (not one byte or grapheme cluster); `*` consumes zero or more.
Backslash quotes the next scalar. A trailing backslash is a literal backslash.
Quote patterns in your shell, for example `'*rust?'`, to avoid shell expansion.
An empty pattern matches only the empty word. Repeated stars never duplicate
results. `wildcard_with_limit(pattern, n)` selects the first `n` sorted matches.

The glob engine uses dynamic programming, not exponential recursive
backtracking. It scans words, so a small result limit does not promise that only
that many tree nodes are visited. Prefer prefix completion for large dictionaries
when your query is a simple prefix.
