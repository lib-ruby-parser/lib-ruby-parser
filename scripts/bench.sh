#!/bin/bash

set -eu

REPEAT=5

function prepare_lib_ruby_parser {
    cargo build --release --example parse --features run-examples
}

function run_lib_ruby_parser {
    for (( x = 1; x <= REPEAT; x += 1));
    do
        echo "Run $x:"
        ./bench/run-lib-ruby-parser.sh
    done
}

function run_ripper {
    echo "Running MRI/ripper"

    for (( x = 1; x <= REPEAT; x += 1));
    do
        echo "Run $x:"
        ./bench/run-ripper.rb
    done
}

prepare_lib_ruby_parser
echo "Running lib-ruby-parser"
run_lib_ruby_parser

echo "--------"

run_ripper
