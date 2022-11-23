#!/bin/bash

set -euo pipefail

source $(pwd)/env/local

git pull

$(pwd)/deploy.sh

