#!/bin/bash

# Ensure at least one argument is provided
if [ "$#" -lt 1 ]; then
    echo "Usage: $0 <crate-name> [-e|-d] [path-to-Cargo.toml]"
    exit 1
fi

# Parse arguments
CRATE_NAME=$1
OPTION=${2:-}
TOML=${3:-}
CARGO_TOML_PATH="${TOML:=Cargo.toml}"

# Extract activated features from Cargo.toml, default to empty list if not present
ACTIVATED_FEATURES=$(toml2json "$CARGO_TOML_PATH" | jq --arg crate "$CRATE_NAME" '
  if .dependencies[$crate] == null then
    []
  elif .dependencies[$crate] | type == "object" and .features then
    .dependencies[$crate].features
  else
    []
  end
')

# Use cargo metadata and jq to group features into enabled and disabled
FEATURES=$(cargo metadata --format-version 1 | jq --argjson enabled "$ACTIVATED_FEATURES" --arg crate "$CRATE_NAME" '
  .packages[] | select(.name == $crate) |
  {
    enabled: .features | keys | map(select(. as $f | $enabled | index($f) != null)),
    disabled: .features | keys | map(select(. as $f | $enabled | index($f) == null))
  }
')

# Output based on the provided option
case "$OPTION" in
  -e)
    echo "$FEATURES" | jq '.enabled'
    ;;
  -d)
    echo "$FEATURES" | jq '.disabled'
    ;;
  "")
    echo "$FEATURES" | jq '.'
    ;;
  *)
    echo "Invalid option: $OPTION"
    echo "Usage: $0 <crate-name> [-e|-d] [path-to-Cargo.toml]"
    exit 1
    ;;
esac

