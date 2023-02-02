# Data structure and resource costs

`Trie` is a set of normalized strings. Each node has an ordered `BTreeMap<char,
TrieNode>` and a terminal bit. `len` counts terminal nodes, including the root
when the empty string is present. Children with no terminal descendants are
pruned on deletion. The case option belongs to the dictionary, not to a query.

Let `L` be the number of input scalars, `d` the maximum branching factor, `V` the
number of visited nodes, and `S` the total bytes of returned words.

- Insert, exact lookup, prefix-path lookup, longest prefix, and deletion take
  `O(L log(d + 1))` tree work. Lowercasing adds input traversal and can expand keys.
- `len` and `is_empty` are constant time.
- Completion visits only the matching subtree, in deterministic order. A shared
  UTF-8 buffer avoids copying every intermediate prefix; traversal is `O(V + S)`
  after prefix lookup. A limit stops after enough terminal words are found, but
  sparse/deep branches can still require more visits than the result count.
- Prefix counts visit matching subtree nodes but do not build result strings.
- Glob matching currently enumerates all keys, then uses a two-row dynamic
  program per candidate. With `P` pattern tokens, time is bounded by
  `O(P * sum(candidate scalar lengths + 1))`; it is not exponential in stars.
  The result limit does not avoid initial whole-dictionary enumeration. Memory
  includes all enumerated keys plus per-candidate character/DP buffers.
- Clone visits the full structure. Removal uses an explicit ancestor stack;
  clone, clear, destruction, and partial-load cleanup avoid call-stack recursion.
- `tree()` is a diagnostic string. Its whitespace output can be quadratic for a
  single deep chain, so avoid rendering huge/untrusted dictionaries as a tree.
  `Debug` is intentionally a bounded metadata summary instead.

Persistence holds returned word strings while writing; it is not a streaming
zero-copy database. Loading has no input size quota, checksum, or authentication.
Apply application-level byte/key limits to untrusted sources. The implementation
prioritizes deterministic behavior and inspectable contracts over a compressed
radix representation, lock-free mutation, or ranked/fuzzy search.

Unicode lowercase can be context-sensitive. Prefix and wildcard-literal queries
normalize their own input, not a hypothetical full candidate; those semantics do
not promise locale-aware or full Unicode caseless search. Read
[query semantics](query-semantics.md) before choosing insensitive mode.
