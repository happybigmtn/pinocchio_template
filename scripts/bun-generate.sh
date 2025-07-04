#!/bin/bash

# Wrapper script for bun generate that preserves the original PWD
# This is needed because bun changes to the project root before running scripts

# Get the directory where bun was called from 
# Bun sets npm_config_local_prefix to the original directory
if [ -n "$npm_config_local_prefix" ]; then
    ORIGINAL_PWD="$npm_config_local_prefix"
elif [ -n "$INIT_CWD" ]; then
    ORIGINAL_PWD="$INIT_CWD"
else
    ORIGINAL_PWD="$PWD"
fi


# Get the script directory and root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Export the original PWD and call the smart-generate script
export ORIGINAL_PWD
exec "$SCRIPT_DIR/smart-generate.sh" "$@"
