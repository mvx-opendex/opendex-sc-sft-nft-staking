#!/bin/bash

set -ue

SCRIPT_DIR=$(dirname $0)
BYTECODE="${SCRIPT_DIR}/../output-docker/opendex-sc-sft-nft-staking-controller/opendex-sc-sft-nft-staking-controller.wasm"
PROXY=https://gateway.multiversx.com
PLAY_API_URL=https://play-api.multiversx.com
MVX_TOOLS_URL=https://tools.multiversx.com
SC_ADDRESS=$(mxpy data load --key=address-mainnet)
CHAIN=1
TEMPLATE_ADDRESS=erd1qqqqqqqqqqqqqpgqxlnvr3m0n5mc8q7d8hyw9pn5z8tvzrpd6avsslv52s


deploy() {
    echo 'You are about to deploy SC on mainnet (Ctrl-C to abort)'
    read answer

    mxpy contract deploy --bytecode=${BYTECODE} \
        --arguments "${TEMPLATE_ADDRESS}" \
        --keyfile=${1} --gas-limit=100000000 --outfile="deploy.interaction.json" \
        --proxy=${PROXY} --chain=${CHAIN} --recall-nonce --send || return

    SC_ADDRESS=$(cat deploy.interaction.json | jq -r .contractAddress)

    mxpy data store --key=address-mainnet --value=${SC_ADDRESS}

    echo ""
    echo "Smart contract address: ${SC_ADDRESS}"
}

upgrade() {
    echo 'You are about to upgrade current SC on mainnet (Ctrl-C to abort)'
    read answer

    mxpy contract upgrade --bytecode=${BYTECODE} --metadata-payable \
        --keyfile=${1} --gas-limit=50000000 --outfile="deploy.interaction.json" \
        --proxy=${PROXY} --chain=${CHAIN} \
        --recall-nonce --send ${SC_ADDRESS} || return

    echo ""
    echo "Smart contract upgraded: ${SC_ADDRESS}"
}

verify() {
    mxpy contract verify "${SC_ADDRESS}" \
        --packaged-src="${SCRIPT_DIR}/../output-docker/opendex-sc-sft-nft-staking-controller/opendex-sc-sft-nft-staking-controller-0.0.0.source.json" \
        --verifier-url="${PLAY_API_URL}" \
        --docker-image="multiversx/sdk-rust-contract-builder:v9.0.0" \
        --keyfile=${1}
}

CMD=$1
shift

$CMD $*
