#!/bin/bash

set -euo pipefail

CONTRACTS_PATH=$(pwd)/../contracts

cd "$CONTRACTS_PATH"/stake_voting && cargo +nightly contract build --release
cp -rf ./target/ink/metadata.json ./target/ink/stake_voting.* /media/sf_Downloads/


STAKE_VOTING_CODE_HASH=$(cargo contract upload --url "$NODE_URL" --suri "$AUTHORITY_SEED" target/ink/stake_voting.wasm --skip-confirm --verbose)
echo "$STAKE_VOTING_CODE_HASH\n\n"

STAKE_VOTING_CODE_HASH=$(echo "$STAKE_VOTING_CODE_HASH" | grep hash | tail -1 | cut -c 14-)
echo "Stake voting code hash: $STAKE_VOTING_CODE_HASH"

STAKE_VOTING=$(cargo contract instantiate --url "$NODE_URL" --suri "$AUTHORITY_SEED" --code-hash "$STAKE_VOTING_CODE_HASH" --constructor default --skip-confirm --verbose)
# STAKE_VOTING=$(cargo contract instantiate --url "wss://ws.test.azero.dev" --suri "0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a" --code-hash "0xd8c7192a5b78d4009f19d8f7c1b88a281592028add3117ad22c4e810fb043a04" --constructor default --skip-confirm)
echo "$STAKE_VOTING\n\n"

STAKE_VOTING=$(echo "$STAKE_VOTING" | grep -A3 "Event Contracts ➜ Instantiated" | grep contract | tail -1 | cut -d ' ' -f11)
echo "Stake voting instance address: $STAKE_VOTING"

cargo contract call --url "$NODE_URL" --suri "$AUTHORITY_SEED" --contract $STAKE_VOTING -m "add_one_stake_operator" \
 --args "{ stake_operator_id: 5Eo5ZxVUGbT6D8cfAvAxQFhzt3ZqBEb5oE8KCWR9vjvTPSMy, name: \"AZF/SHANNON\", own_staked: 642_722, other_staked: 3_218_000, commission: 0, total_stakers: 618, vote_points: 0 }" --skip-confirm

cargo contract call --url "$NODE_URL" --suri "$AUTHORITY_SEED" --contract $STAKE_VOTING -m "add_one_stake_operator" \
 --args "{ stake_operator_id: 5GW5kbpuYn8Wa2253xLNLn9dZYWJUPJmW7VwmjnziDWdGxiX, name: \"AZF/SIERPINSKI\", own_staked: 73_606, other_staked: 334_565, commission: 0, total_stakers: 509, vote_points: 0 }" --skip-confirm

cargo contract call --url "$NODE_URL" --suri "$AUTHORITY_SEED" --contract $STAKE_VOTING -m "add_one_stake_operator" \
 --args "{ stake_operator_id: 5CGTtuqDbBQokPQjpa4mQyNKyvYxKpgtZEskDkJxzho1NhbM, name: \"AZF/CANTOR\", own_staked: 229_798, other_staked: 567_941, commission: 0, total_stakers: 515, vote_points: 0 }" --skip-confirm

cargo contract call --url "$NODE_URL" --suri "$AUTHORITY_SEED" --contract $STAKE_VOTING -m "add_one_stake_operator" \
 --args "{ stake_operator_id: 5HYzfrjAMGB6zWW3oTg7dhGdWB8cawyU84fCpGar9QhupweS, name: \"AZF/HILBERT\", own_staked: 140_053, other_staked: 303_097, commission: 0, total_stakers: 515, vote_points: 0 }" --skip-confirm

cargo contract call --url "$NODE_URL" --suri "$AUTHORITY_SEED" --contract $STAKE_VOTING -m "add_one_stake_operator" \
 --args "{ stake_operator_id: 5Dkh7kuPm4NMfkmDG1LaVZVWXW3WHYwh7BKEFfNvPiGDrARH, name: \"AZF/FERMAT\", own_staked: 142_356, other_staked: 498_871, commission: 0, total_stakers: 511, vote_points: 0 }" --skip-confirm

cargo contract call --url "$NODE_URL" --suri "$AUTHORITY_SEED" --contract $STAKE_VOTING -m "add_one_stake_operator" \
 --args "{ stake_operator_id: 5HNnDD5djTaiUt3A6yf6f1E9oDiM5w5fcNBTLLCoMKf1TEdS, name: \"AZF/RAMANUJAN\", own_staked: 300_787, other_staked: 699_972, commission: 0, total_stakers: 515, vote_points: 0 }" --skip-confirm

cargo contract call --url "$NODE_URL" --suri "$AUTHORITY_SEED" --contract $STAKE_VOTING -m "add_one_stake_operator" \
 --args "{ stake_operator_id: 5Grh6bLQmoxinEeiijAfSbGYrYiKhxnxcM2m96s5A64VyAiF, name: \"AZF/GALOIS\", own_staked: 73_132, other_staked: 243_081, commission: 0, total_stakers: 504, vote_points: 0 }" --skip-confirm

cargo contract call --url "$NODE_URL" --suri "$AUTHORITY_SEED" --contract $STAKE_VOTING -m "add_one_stake_operator" \
 --args "{ stake_operator_id: 5DATX2UZZgxAsumbVEsmup2q6LR9Bn81F7KW7PsShgUw8t12, name: \"AZF/RIEMANN\", own_staked: 119_877, other_staked: 202_892, commission: 0, total_stakers: 517, vote_points: 0 }" --skip-confirm

cargo contract call --url "$NODE_URL" --suri "$AUTHORITY_SEED" --contract $STAKE_VOTING -m "add_one_stake_operator" \
 --args "{ stake_operator_id: 5FnyjESMB4EBQn1W1vnNKZ5oVUYUmQbTPG4hZbJJm8697TKt, name: \"AZF/CAUCHY\", own_staked: 50_355, other_staked: 282_103, commission: 0, total_stakers: 500, vote_points: 0 }" --skip-confirm

cargo contract call --url "$NODE_URL" --suri "$AUTHORITY_SEED" --contract $STAKE_VOTING -m "add_one_stake_operator" \
 --args "{ stake_operator_id: 5GN3rbR41UYWtjoxeuyvBfWEPopH4C2R4z7qhtz2ysF5hmrt, name: \"AZF/LAPLACE\", own_staked: 224_547, other_staked: 859_746, commission: 0, total_stakers: 502, vote_points: 0 }" --skip-confirm