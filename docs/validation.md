# Local acceptance evidence

The implemented library, binary, examples, and documentation were exercised locally
with `rustc 1.99.0` and `cargo 1.99.0`. This records actual command execution, not
mocked output or a claim that historical commit timestamps are execution dates.

## Passed gates

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets --all-features`: **72 passed, 0 failed**.
- `cargo test --doc`: **4 passed, 0 failed**.
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`
- `cargo test --release --all-targets --all-features`: **72 passed, 0 failed**.
- `cargo run --quiet --example dictionary`: output `python, rust`.
- `bash scripts/smoke-cli.sh`: insert, normalization, completion, wildcard, saving,
  deletion, missing-word exit status, and owned temporary-directory cleanup passed.
- Repository-relative links in the maintained Markdown guides resolve to files.

Tests include deterministic randomized mutations against `BTreeSet`, prefix-query
set models, glob differential checks against a small recursive reference matcher,
UTF-8/control-character snapshot round trips, strict corruption handling, partial
I/O failures, file-replacement safety, subprocess CLI behavior, and isolated
100,000-scalar stack-safety regressions. A subprocess crash is treated as a failure,
not silently skipped.

## Claim boundaries

- These are local results. The new CI workflow has not been pushed or run remotely.
- No package/release is published. No benchmark throughput is claimed.
- No old compiler/MSRV or every-platform support claim follows from this one
  toolchain/host run.
- Lowercase keys are not full Unicode case folding; resource and durability limits
  remain as documented in the query, complexity, and file-format guides.
- Commit dates are deliberately reconstructed; see [history provenance](../HISTORY.md).
