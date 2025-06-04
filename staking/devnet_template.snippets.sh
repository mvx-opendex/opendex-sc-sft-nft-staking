#!/bin/bash

set -ue

SCRIPT_DIR=$(dirname $0)
BYTECODE="${SCRIPT_DIR}/../output-docker/opendex-sc-sft-nft-staking/opendex-sc-sft-nft-staking.wasm"
PROXY=https://devnet-gateway.multiversx.com
PLAY_API_URL=https://devnet-play-api.multiversx.com
SC_ADDRESS=$(mxpy data load --key=address-devnet-template)
CHAIN=D
DEAD_ADDRESS=erd1deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaqtv0gag


deploy() {
    echo 'You are about to deploy SC on devnet (Ctrl-C to abort)'
    read answer

    mxpy contract deploy --bytecode=${BYTECODE} --metadata-payable \
        --arguments "0x" "0x" "${DEAD_ADDRESS}" "0" "${DEAD_ADDRESS}" \
        --keyfile=${1} --gas-limit=100000000 --outfile="deploy.interaction.json" \
        --proxy=${PROXY} --chain=${CHAIN} --recall-nonce --send || return

    SC_ADDRESS=$(cat deploy.interaction.json | jq -r .contractAddress)

    mxpy data store --key=address-devnet-template --value=${SC_ADDRESS}

    echo ""
    echo "Smart contract address: ${SC_ADDRESS}"
}

upgrade() {
    echo 'You are about to upgrade current SC on devnet (Ctrl-C to abort)'
    read answer

    mxpy contract upgrade --bytecode=${BYTECODE} --metadata-payable \
        --keyfile=${1} --gas-limit=100000000 --outfile="deploy.interaction.json" \
        --proxy=${PROXY} --chain=${CHAIN} --recall-nonce --send ${SC_ADDRESS} || return

    echo ""
    echo "Smart contract upgraded: ${SC_ADDRESS}"
}

verify() {
    mxpy contract verify "${SC_ADDRESS}" \
        --packaged-src="${SCRIPT_DIR}/../output-docker/opendex-sc-sft-nft-staking/opendex-sc-sft-nft-staking-0.0.0.source.json" \
        --verifier-url="${PLAY_API_URL}" \
        --docker-image="multiversx/sdk-rust-contract-builder:v9.0.0" \
        --keyfile=${1}
}

CMD=$1
shift

$CMD $*
