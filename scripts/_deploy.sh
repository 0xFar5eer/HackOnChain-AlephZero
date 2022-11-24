#!/bin/bash

set -euo pipefail

CONTRACTS_PATH=$(pwd)/../contracts

cd "$CONTRACTS_PATH"/stake_voting && cargo +nightly contract build --release
cp -rf ./target/ink/metadata.json ./target/ink/stake_voting.* /media/sf_Downloads/


STAKE_VOTING_CODE_HASH=$(cargo contract upload --quiet --url "$NODE_URL" --suri "$AUTHORITY_SEED" target/ink/stake_voting.wasm --skip-confirm)
echo "$STAKE_VOTING_CODE_HASH\n\n"

STAKE_VOTING_CODE_HASH=$(echo "$STAKE_VOTING_CODE_HASH" | grep hash | tail -1 | cut -c 14-)
echo "Stake voting code hash: $STAKE_VOTING_CODE_HASH"

STAKE_VOTING=$(cargo contract instantiate --url "$NODE_URL" --suri "$AUTHORITY_SEED" --code-hash "$STAKE_VOTING_CODE_HASH" --constructor default --skip-confirm)
# STAKE_VOTING=$(cargo contract instantiate --url "wss://ws.test.azero.dev" --suri "0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a" --code-hash "0xd8c7192a5b78d4009f19d8f7c1b88a281592028add3117ad22c4e810fb043a04" --constructor default --skip-confirm)
echo "$STAKE_VOTING\n\n"

STAKE_VOTING=$(echo "$STAKE_VOTING" | grep -A3 "Event Contracts ➜ Instantiated" | grep contract | tail -1 | cut -d ' ' -f11)
echo "Stake voting instance address: $STAKE_VOTING"
