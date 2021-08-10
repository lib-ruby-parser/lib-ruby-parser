#!/bin/bash

set -eux

export RUST_BACKTRACE=1

rustup default nightly

# cargo test # --features "onig"

cargo clean
./scripts/test-c.sh
exit 1

cargo clean

run_cpp_tests() {
    ASAN_OPTIONS=detect_leaks=1 \
    RUSTFLAGS=-Zsanitizer=address \
    LIB_RUBY_PARSER_PTR_SIZE=8 \
    LIB_RUBY_PARSER_MAYBE_PTR_SIZE=8 \
    LIB_RUBY_PARSER_LIST_SIZE=24 \
    LIB_RUBY_PARSER_STRING_PTR_SIZE=8 \
    LIB_RUBY_PARSER_MAYBE_STRING_PTR_SIZE=8 \
    LIB_RUBY_PARSER_SHARED_BYTE_LIST_SIZE=16 \
    LIB_RUBY_PARSER_BYTES_SIZE=24 \
    LIB_RUBY_PARSER_TOKEN_SIZE=56 \
    LIB_RUBY_PARSER_SOURCE_LINE_SIZE=24 \
    LIB_RUBY_PARSER_ERROR_LEVEL_SIZE=4 \
    LIB_RUBY_PARSER_LOC_SIZE=16 \
    LIB_RUBY_PARSER_MAYBE_LOC_SIZE=24 \
    LIB_RUBY_PARSER_COMMENT_TYPE_SIZE=4 \
    LIB_RUBY_PARSER_COMMENT_SIZE=24 \
    LIB_RUBY_PARSER_MAGIC_COMMENT_KIND_SIZE=4 \
    LIB_RUBY_PARSER_MAGIC_COMMENT_SIZE=40 \
    LIB_RUBY_PARSER_DIAGNOSTIC_MESSAGE_SIZE=24 \
        cargo test --features "compile-with-external-structures,link-with-external-cpp-structures" "$@"
}

# Linking fails on building doctests with ASAN enabled
run_cpp_tests --lib -- test_size
run_cpp_tests --lib
run_cpp_tests --test parser_test
run_cpp_tests --test lexer_test
