#!/bin/sh

set -eu

DOCKER_OUTPUT_FOLDER=./output-docker

cargo install cargo-llvm-cov

sc-meta all proxy

sc-meta all build

sc-meta test-gen

sc-meta test

if [ -d "${DOCKER_OUTPUT_FOLDER}" ]
then
    echo "Delete ${DOCKER_OUTPUT_FOLDER} ?"
    read -p "Enter to continue (Crtl-C to abort)" ANSWER
fi

rm -rf ${DOCKER_OUTPUT_FOLDER}

mxpy contract reproducible-build --docker-image="multiversx/sdk-rust-contract-builder:v8.0.1"
