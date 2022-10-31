#!/bin/sh
set -euo pipefail

# Determine the platforms default target triple if not set manually
if test -z "${TARGET:-}"; then
    TARGET=`rustup default | cut -d" " -f1 | cut -d"-" -f2,3,4`
fi

# Configure cargo and rust
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="target/$TARGET/debug/cargo-test.profraw"
cargo test --target="$TARGET" --features=trace

# Create coverage report
grcov "target/$TARGET/debug/" --binary-path "target/$TARGET/debug/"  -s . \
    -t html --branch --ignore-not-existing -o "target/coverage/"

# Display coverage path
PWD=`pwd`
echo "Coverage is at file://$PWD/target/coverage/index.html"
