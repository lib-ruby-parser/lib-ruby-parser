#!/bin/bash

set -eu

REPEAT=5

function prepare_lib_ruby_parser {
    cargo build --release --bin parse --features=bin-parse
}

function run_lib_ruby_parser {
    for (( x = 1; x <= REPEAT; x += 1));
    do
        echo "Run $x:"
        ./target/release/parse --print=N --drop-tokens --run-timer --glob "gems/repos/**/*.rb"
    done
}

function run_ripper {
    echo "Running MRI/ripper"

    for (( x = 1; x <= REPEAT; x += 1));
    do
        echo "Run $x:"
        ./scripts/bench.rb
    done
}

prepare_lib_ruby_parser
echo "Running lib-ruby-parser"
run_lib_ruby_parser

echo "--------"

run_ripper
