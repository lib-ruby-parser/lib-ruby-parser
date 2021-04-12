#!/bin/bash

REPEAT=5
export WORKDIR=$PWD

function prepare_lib_ruby_parser {
    cd $WORKDIR/examples && cargo build --release --example parse
}

function run_lib_ruby_parser {
    echo "Running lib-ruby-parser"

    for (( x = 1; x <= REPEAT; x += 1));
    do
        echo "Run $x:"
        cd $WORKDIR
        ./bench/run-lib-ruby-parser.sh
    done
}

function run_ripper {
    echo "Running MRI/ripper"

    for (( x = 1; x <= REPEAT; x += 1));
    do
        echo "Run $x:"
        cd $WORKDIR
        ./bench/run-ripper.rb
    done
}

prepare_lib_ruby_parser

run_lib_ruby_parser

echo "--------"

run_ripper
