#!/bin/bash

set -eux

export RUST_BACKTRACE=1

rustup default nightly

# cargo test # --features "onig"

cargo clean
bash ./scripts/test-c.sh

cargo clean
bash ./scripts/test-cpp.sh
