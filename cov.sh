#!/bin/sh

export CARGO_INCREMENTAL=0
export RUSTC_BOOTSTRAP=1

export RUSTFLAGS=-Cinstrument-coverage
export RUSTFLAGS="$RUSTFLAGS -Zprofile"
export RUSTFLAGS="$RUSTFLAGS -Ccodegen-units=1"
export RUSTFLAGS="$RUSTFLAGS -Copt-level=0"
export RUSTFLAGS="$RUSTFLAGS -Clink-dead-code"
export RUSTFLAGS="$RUSTFLAGS -Coverflow-checks=off"
export RUSTFLAGS="$RUSTFLAGS -Zpanic_abort_tests"
export RUSTFLAGS="$RUSTFLAGS -Cpanic=abort"

cargo build --verbose $CARGO_OPTIONS

export LLVM_PROFILE_FILE=yanagitani.profraw

cargo test --verbose $CARGO_OPTIONS -- --include-ignored

grcov . \
  --source-dir . \
  --binary-path ./target/debug/ \
  --output-type html \
  --branch \
  --ignore-not-existing \
  --output-path ./target/debug/coverage/
