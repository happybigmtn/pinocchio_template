#!/bin/bash

# Program-specific deployment script
# This is a convenience script that calls the main deploy.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$ROOT_DIR"

# Extract program name from directory structure
PROGRAM_NAME=$(basename "$SCRIPT_DIR")

# Call main deployment script
exec "./scripts/deploy.sh" "$PROGRAM_NAME" "$@"
