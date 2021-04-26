#!/bin/bash

set -eu

export RUST_BACKTRACE=1

rustup default stable
cargo test --features "onig"

rustup default nightly

export ASAN_OPTIONS=detect_leaks=1
export RUSTFLAGS=-Zsanitizer=address

cargo test --features "compile-with-external-structures,link-external-c-structures" --test parser_test
cargo test --features "compile-with-external-structures,link-external-c-structures" -- containers

cargo test --features "compile-with-external-structures,link-external-cpp-structures" --test parser_test
cargo test --features "compile-with-external-structures,link-external-cpp-structures" -- containers
