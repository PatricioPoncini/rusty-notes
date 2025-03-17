#!/usr/bin/env bash

set -eou pipefail

docker compose up wait-for-db-to-test -d

cargo test