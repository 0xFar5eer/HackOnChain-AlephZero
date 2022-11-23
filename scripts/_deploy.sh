#!/bin/bash

set -euo pipefail

CONTRACTS_PATH=$(pwd)/../contracts

cd "$CONTRACTS_PATH"/stake_voting && cargo +nightly contract build --release


STAKE_VOTING_CODE_HASH=$(cargo contract upload --quiet --url "$NODE_URL" --suri "$AUTHORITY_SEED" target/ink/stake_voting.wasm --skip-confirm)
echo "$STAKE_VOTING_CODE_HASH\n\n"

STAKE_VOTING_CODE_HASH=$(echo "$STAKE_VOTING_CODE_HASH" | grep hash | tail -1 | cut -c 14-)
echo "Stake voting code hash: $STAKE_VOTING_CODE_HASH"

STAKE_VOTING=$(cargo contract instantiate --url "$NODE_URL" --suri "$AUTHORITY_SEED" --code-hash "$STAKE_VOTING_CODE_HASH" --skip-confirm)

# STAKE_VOTING=$(echo "$STAKE_VOTING" | grep -A3 "Event Contracts ➜ Instantiated" | grep contract | tail -1 | cut -d ' ' -f11)
# echo "Bulletin board instance address: $STAKE_VOTING"
# HIGHLIGHTED_POSTS_INSTANCE=$(cargo contract call --url "$NODE_URL" --suri "$AUTHORITY_SEED" --contract "$STAKE_VOTING" -m get_highlights_board --skip-confirm --quiet --dry-run | grep Data | grep -Poe "Some\(\K[a-zA-Z0-9]+")
# echo "Highlighted posts instance address: $STAKE_VOTING"
