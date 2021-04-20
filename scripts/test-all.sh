#!/bin/bash

set -eu

rustup default stable
cargo test --features "onig"

rustup default nightly
cargo test --features "c-structures" --test parser_test
