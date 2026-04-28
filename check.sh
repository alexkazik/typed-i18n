#!/bin/bash

set -e # exit on error
set -x # show commands executed

#
# Perform a few simple checks ahead of a PR
#

# Usage: `./check.sh` or `./check.sh <toolchain>`
# If the toolchain is omitted `+nightly`,`+stable` and `+1.58.1` is used, `+stable` or `+beta` are the most common alternatives

# 2024 edition: 1.85
TOOLCHAIN=${1:-+1.85.0}
echo Using toolchain $TOOLCHAIN

# use crates available at this rust version
cargo $TOOLCHAIN update

# builds (alloc, nothing)
cargo $TOOLCHAIN build --release --all-features --tests
cargo $TOOLCHAIN build --release --no-default-features --tests

TOOLCHAIN=${1:-+nightly}
echo Using toolchain $TOOLCHAIN

# builds (alloc, nothing)
cargo $TOOLCHAIN build --release --all-features --tests
cargo $TOOLCHAIN build --release --no-default-features --tests

# clippy (alloc, nothing)
cargo $TOOLCHAIN clippy --release --all-features --tests -- -D warnings
cargo $TOOLCHAIN clippy --release --no-default-features -- -D warnings

# update formatting
for width in 500 400 300 200 150 130 110
do
  cargo $TOOLCHAIN fmt --all -- --config max_width=$width
done
cargo $TOOLCHAIN fmt --all

# update readme
( cd typed-i18n && cargo rdme --force )

# create docs
if test "$TOOLCHAIN" = "+nightly"
then
  RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc -p typed-i18n --all-features
else
  echo "Skipping 'cargo doc' with doc_cfg since it's only available on nightly"
fi

TOOLCHAIN=${1:-+stable}
echo Using toolchain $TOOLCHAIN

# tests
cargo $TOOLCHAIN test --locked --release --all-features -- --include-ignored
cargo $TOOLCHAIN test --locked --release --no-default-features -- --include-ignored
cargo $TOOLCHAIN test --locked --release --no-default-features --features alloc -- --include-ignored
cargo $TOOLCHAIN test --locked --release --no-default-features --features json -- --include-ignored
cargo $TOOLCHAIN test --locked --release --no-default-features --features yaml -- --include-ignored

# build the examples
( cd examples && cargo $TOOLCHAIN build )

if command -v typos >/dev/null 2>&1
then
  typos
else
  echo "typos check not run, see https://github.com/crate-ci/typos if interested"
fi
