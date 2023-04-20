# Snapshot format v1

A snapshot is UTF-8. Its first line is exactly `TRIE\t1\tS\n` for a sensitive
trie or `TRIE\t1\tI\n` for a Unicode-lowercase trie. Here `\t` and `\n`
denote actual tab and LF bytes in the header.

Subsequent LF-terminated records each contain one escaped word. The empty word
is an empty record. Backslash, LF, CR, tab, and NUL inside a word are encoded as
`\\`, `\n`, `\r`, `\t`, and `\0` respectively. Other scalars are literal.
The writer sorts words lexicographically and emits no duplicates. The reader
accepts reordered or duplicate records with set semantics and normalizes records
when the header selects insensitive mode.

Missing or unknown headers, invalid UTF-8, unknown/dangling escapes, and a final
record without LF return `io::ErrorKind::InvalidData`. CRLF is not a supported
snapshot record terminator; use the writer rather than a platform text editor.
There is no checksum or authenticated envelope. A valid-looking edit can change
a dictionary; only load trusted snapshots.

## APIs and write safety

- `to_writer(writer)` writes and flushes the stream; errors propagate.
- `from_reader(bufread)` builds a new trie and returns nothing on failure.
- `load(path)` opens a buffered reader and uses the same parser.
- `save(path)` reserves a new sibling temporary file, writes and syncs it, then
  renames it over the destination. Earlier failures leave the destination alone.
  Temporary files are removed on normal error paths. A process crash can leave a
  temporary file behind. The parent directory is not fsynced; this is atomic
  replacement, not a promise of power-loss durability. Existing permissions are
  not preserved. Concurrent saves use last-successful-rename-wins semantics.

Plain word lists are a separate interchange format, not snapshots: they do not
carry case options or escape control characters. See the import/export APIs.
