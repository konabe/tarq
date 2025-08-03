#!/bin/bash

CRATE_NAME=$1
CRATE_VERSION=$(cargo metadata --manifest-path $CRATE_NAME/Cargo.toml --no-deps --format-version 1 | jq -r '.packages[0].version')
if curl --silent "https://crates.io/api/v1/crates/${CRATE_NAME}" | jq -e ".versions[] | select(.num == \"${CRATE_VERSION}\")" > /dev/null; then
  echo "true"
else
  echo "false"
fi