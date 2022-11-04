#!/usr/bin/env bash
set -euo pipefail
repo_dir=$(CDPATH= cd -- "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repo_dir"
scratch_root=${TMPDIR:-"$HOME/.cache"}
mkdir -p "$scratch_root"
work=$(mktemp -d "$scratch_root/trie-rs-smoke.XXXXXX")
trap 'rm -rf -- "$work"' EXIT
run() { cargo run --quiet -- "$@"; }
db="$work/dictionary.trie"
[[ $(run -i --save "$db" insert Rust RUST Ruby) == 2 ]]
[[ $(run --load "$db" search RUST) == true ]]
[[ $(run --load "$db" --limit 1 complete R) == ruby ]]
[[ $(run --load "$db" match 'r?st') == rust ]]
[[ $(run --load "$db" --save "$db" delete RUBY missing) == 1 ]]
[[ $(run --load "$db" len) == 1 ]]
if run --load "$db" search missing > "$work/absent"; then
    printf 'expected search to exit 1 for a missing word\n' >&2
    exit 1
else
    status=$?
    [[ $status -eq 1 ]]
fi
[[ $(< "$work/absent") == false ]]
printf 'CLI smoke passed: insert, normalize, complete, wildcard, save, delete, exit status\n'
