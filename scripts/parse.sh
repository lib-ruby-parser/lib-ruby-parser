#!/bin/bash

export WORKDIR=$PWD
RUST_BACKTRACE=1 cd $WORKDIR/examples && cargo run --example parse "$@"
