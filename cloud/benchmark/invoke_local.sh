#!/usr/bin/env bash

set -e -u -o pipefail

export RUN_LOCALLY=1
cargo run --release --bin chessmove-benchmark-lambda
