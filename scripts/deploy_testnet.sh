#!/bin/bash

set -euo pipefail

source $(pwd)/env/testnet

$(pwd)/deploy.sh

