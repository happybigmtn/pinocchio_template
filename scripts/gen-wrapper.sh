#!/bin/bash

# Wrapper script to detect working directory for smart-generate.sh
# When bun runs this, it changes to package root, but we can detect the intended directory

# Get the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# The key insight: bun preserves some environment variables that can help us
# detect the original working directory. We need to check multiple sources.

CURRENT_DIR="$PWD"

# Check if we have environment variables that might indicate the original directory
if [ -n "$ORIG_PWD" ]; then
    CURRENT_DIR="$ORIG_PWD"
elif [ -n "$INIT_CWD" ]; then
    CURRENT_DIR="$INIT_CWD"
fi

# If we have explicit arguments, use them instead of auto-detection
if [ -n "$1" ]; then
    # Pass arguments directly to smart-generate.sh
    "$SCRIPT_DIR/smart-generate.sh" "$@"
    exit $?
fi

# If we're in the root directory, try some heuristics to detect the intended program
if [[ "$(realpath --relative-to="$ROOT_DIR" "$CURRENT_DIR" 2>/dev/null || echo ".")" == "." ]]; then
    
    # Heuristic 1: Check if there's only one program directory - if so, use it
    program_dirs=($(find "$ROOT_DIR" -mindepth 2 -maxdepth 2 -type d -path "*/basics/*" -o -path "*/tokens/*" -o -path "*/compression/*" -o -path "*/oracles/*" | sort))
    
    if [ ${#program_dirs[@]} -eq 1 ]; then
        # Only one program exists, use it
        CURRENT_DIR="${program_dirs[0]}"
        echo "Auto-detected single program: $(basename "$(dirname "$CURRENT_DIR")"/$(basename "$CURRENT_DIR"))"
    else
        # Multiple programs exist, try heuristic 2: find most recently modified
        most_recent_dir=$(find "$ROOT_DIR" -mindepth 2 -maxdepth 2 -type d \( -path "*/basics/*" -o -path "*/tokens/*" -o -path "*/compression/*" -o -path "*/oracles/*" \) -printf '%T@ %p\n' 2>/dev/null | sort -n | tail -1 | cut -d' ' -f2- 2>/dev/null)
        
        if [ -n "$most_recent_dir" ] && [ -d "$most_recent_dir" ]; then
            CURRENT_DIR="$most_recent_dir"
            echo "Auto-detected most recently modified program: $(basename "$(dirname "$CURRENT_DIR")"/$(basename "$CURRENT_DIR"))"
        else
            # Fallback: show available programs
            echo "Auto-detection failed. Available programs:"
            printf '%s\n' "${program_dirs[@]}" | sed "s|$ROOT_DIR/||" | sort
            echo ""
            echo "Please specify the program name:"
            echo "  bun gen my-program --category=basics"
            echo "Or run from the program directory."
            exit 1
        fi
    fi
fi

# Set the detected directory and call smart-generate.sh
INIT_CWD="$CURRENT_DIR" "$SCRIPT_DIR/smart-generate.sh" "$@"
