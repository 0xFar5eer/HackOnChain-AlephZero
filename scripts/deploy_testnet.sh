#!/bin/bash

set -euo pipefail

source $(pwd)/env/testnet

git pull

$(pwd)/deploy.sh

