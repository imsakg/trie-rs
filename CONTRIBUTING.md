# Contributing

The crate uses Rust 2021 and has no third-party runtime or development dependencies.
A current stable Rust toolchain with `rustfmt` and `clippy` is sufficient.

## Local quality gate

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo test --doc
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo run --quiet --example dictionary
```

The workflow in `.github/workflows/ci.yml` declares Linux/macOS/Windows checks.
A local pass is not proof that remote CI has run; inspect actual workflow results
before making a cross-platform claim.

## Making a change

Write a focused regression first and demonstrate that it fails for the intended
reason. Implement the smallest behavior, then run the focused test and full gate.
Keep tests deterministic. The generated set/glob models use fixed seeds or exhaustive
small inputs, so a failure is reproducible without a new test dependency.

- Keep tree algorithms iterative. Arbitrarily long valid keys must not cause
  recursive clone, removal, clear, destruction, or error-cleanup stack overflows.
- Preserve UTF-8 scalar boundaries and the normalization policy of each trie.
- Keep snapshot parsing strict and versioned. Do not silently reinterpret a corrupt
  snapshot as a plain word list.
- Reserve sibling temporary files with `create_new`; clean up on failure and disarm
  cleanup after successful rename. Never let a destination alias its temporary file.
- Keep CLI exit codes and stdout/stderr separation stable. Mutations do not touch
  snapshots unless `--save` is explicit.
- Use small, descriptive commits for actual changes. [HISTORY.md](HISTORY.md)
  explains the deliberately reconstructed timestamps in this local expansion;
  new contributors should use their real timestamps normally.

Do not add fuzzy/ranked search, another persistence backend, or locale-specific
case folding incidentally. Those change product and format contracts and deserve
separate design and tests.
