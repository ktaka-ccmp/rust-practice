#!/bin/bash

# Ensure at least one argument is provided
if [ "$#" -lt 1 ]; then
    echo "Usage: $0 <crate-name> "
    exit 1
fi

# Parse arguments
CRATE_NAME=$1
cargo add $CRATE_NAME --features=

