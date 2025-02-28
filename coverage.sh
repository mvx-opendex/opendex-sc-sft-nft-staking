#!/bin/sh

set -eu

mkdir -p coverage

cargo llvm-cov

cargo llvm-cov report --lcov --output-path coverage/lcov.info
