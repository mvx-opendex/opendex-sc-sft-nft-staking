#!/bin/sh

set -eu

cargo llvm-cov

cargo llvm-cov report --lcov --output-path coverage/lcov.info
