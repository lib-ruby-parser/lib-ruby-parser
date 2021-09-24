#!/bin/bash

set -eux

export RUST_BACKTRACE=1
export ASAN_OPTIONS=detect_leaks=1
export RUSTFLAGS=-Zsanitizer=address
export LIB_RUBY_PARSER_SIZES_FILEPATH="$PWD/external/cpp/sizes-out"

rustup default nightly

run_c_tests() {
    cargo test --features "compile-with-external-structures,link-with-external-cpp-structures" "$@" -- --nocapture
}

# Linking fails on building doctests with ASAN enabled
run_c_tests --lib -- containers
run_c_tests --lib
