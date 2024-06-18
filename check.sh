#!/bin/bash

#
# Perform a few simple checks ahead of a PR
#

# Usage: `./check.sh` or `./check.sh <toolchain>`
# If the toolchain is omitted `+nightly`,`+stable` and `+1.58.1` is used, `+stable` or `+beta` are the most common alternatives

# 2021 edition: 1.56; implicit named arguments: 1.58, hashbrown: 1.63, serde_yaml: 1.64
TOOLCHAIN=${1:-+1.64.0}
echo Using toolchain $TOOLCHAIN

# use crates available at this rust version
cargo $TOOLCHAIN update

# builds (alloc, nothing)
cargo $TOOLCHAIN build --release --all-features --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features || exit 1

TOOLCHAIN=${1:-+nightly}
echo Using toolchain $TOOLCHAIN

# builds (alloc, nothing)
cargo $TOOLCHAIN build --release --all-features --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features --tests || exit 1

# clippy (alloc, nothing)
cargo $TOOLCHAIN clippy --release --all-features --tests -- -D warnings || exit 1
cargo $TOOLCHAIN clippy --release --no-default-features -- -D warnings || exit 1

# update formatting
cargo $TOOLCHAIN fmt --all || exit 1

# update readme
( cd typed-i18n && cargo rdme --force ) || exit 1

# create docs
if test "$TOOLCHAIN" = "+nightly"
then
  RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc -p typed-i18n --all-features || exit 1
else
  echo "Skipping 'cargo doc' with doc_cfg since it's only available on nightly"
fi

TOOLCHAIN=${1:-+stable}
echo Using toolchain $TOOLCHAIN

# tests
cargo $TOOLCHAIN test --locked --release --all-features -- --include-ignored || exit 1
cargo $TOOLCHAIN test --locked --release --no-default-features --lib --bins --tests -- --include-ignored || exit 1

# build the examples
( cd examples && cargo $TOOLCHAIN b ) || exit 1
