#!/bin/bash

export WORKDIR=$PWD
RUST_BACKTRACE=1 cargo fuzz run parse --jobs=8 -- -max_len=50
