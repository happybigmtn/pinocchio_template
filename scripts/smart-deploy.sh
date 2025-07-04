#!/bin/bash

# Smart deploy script for Pinocchio programs
# Automatically detects program name from current directory and deploys
# Usage: ./smart-deploy.sh [--network=devnet|testnet|mainnet]

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
NETWORK="devnet"

# Get the current directory (use ORIGINAL_PWD if available, fallback to PWD)
if [ -n "$ORIGINAL_PWD" ]; then
    CURRENT_DIR="$ORIGINAL_PWD"
else
    CURRENT_DIR="$PWD"
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Function to detect program name and category from current directory
detect_program_info() {
    local rel_path=$(realpath --relative-to="$ROOT_DIR" "$CURRENT_DIR" 2>/dev/null || echo "$CURRENT_DIR")
    
    # Check if we're in a program directory (e.g., basics/account_data)
    if [[ "$rel_path" =~ ^([^/]+)/([^/]+)$ ]]; then
        CATEGORY="${BASH_REMATCH[1]}"
        PROGRAM_NAME="${BASH_REMATCH[2]}"
        echo -e "${GREEN}Detected program: $CATEGORY/$PROGRAM_NAME${NC}"
        return 0
    fi
    
    # Check if we're in the root directory and need to specify program
    if [[ "$rel_path" == "." || "$rel_path" == "" ]]; then
        echo -e "${RED}Error: Please run this command from a program directory (e.g., basics/account_data)${NC}"
        echo -e "${YELLOW}Or specify the program name: bun deploy [program_name] --category=[category]${NC}"
        exit 1
    fi
    
    return 1
}

# Function to show help
show_help() {
    echo "Usage: $0 [program_name] [--network=devnet|testnet|mainnet] [--category=basics|tokens|compression|oracles]"
    echo ""
    echo "Smart deploy script for Pinocchio programs"
    echo ""
    echo "When run from a program directory (e.g., basics/account_data):"
    echo "  $0                           # Auto-detects program and deploys to devnet"
    echo "  $0 --network=testnet         # Auto-detects program and deploys to testnet"
    echo ""
    echo "When run with explicit arguments:"
    echo "  $0 my_program                          # Deploy basics/my_program to devnet"
    echo "  $0 token_mint --category=tokens        # Deploy tokens/token_mint to devnet"
    echo "  $0 my_program --network=mainnet        # Deploy to mainnet"
    echo ""
    echo "The script will:"
    echo "  1. Detect or use the specified program"
    echo "  2. Call the main deploy.sh script with appropriate parameters"
    echo "  3. Handle all deployment steps automatically"
}

# Main execution
main() {
    echo -e "${GREEN}🚀 Smart Deploy for Pinocchio Programs${NC}"
    echo ""
    
    # Parse command line arguments
    PROGRAM_NAME=""
    CATEGORY=""
    
    for arg in "$@"; do
        case $arg in
            --network=*)
                NETWORK="${arg#*=}"
                ;;
            --category=*)
                CATEGORY="${arg#*=}"
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            -*)
                # Skip other flags
                ;;
            *)
                # This should be the program name
                if [ -z "$PROGRAM_NAME" ]; then
                    PROGRAM_NAME="$arg"
                fi
                ;;
        esac
    done
    
    # If no program name specified, auto-detect from directory
    if [ -z "$PROGRAM_NAME" ]; then
        detect_program_info
    else
        # Use specified program name
        if [ -z "$CATEGORY" ]; then
            CATEGORY="basics"  # Default category
        fi
        echo -e "${GREEN}Using specified program: $CATEGORY/$PROGRAM_NAME${NC}"
    fi
    
    # Validate that we have a program name and category
    if [ -z "$PROGRAM_NAME" ] || [ -z "$CATEGORY" ]; then
        echo -e "${RED}Error: Could not determine program name and category${NC}"
        exit 1
    fi
    
    # Change to root directory to call deploy script
    cd "$ROOT_DIR"
    
    # Call the main deploy script with detected parameters
    echo -e "${BLUE}Calling deploy script: ./scripts/deploy.sh $PROGRAM_NAME --network=$NETWORK --category=$CATEGORY${NC}"
    echo ""
    
    chmod +x "./scripts/deploy.sh"
    exec "./scripts/deploy.sh" "$PROGRAM_NAME" "--network=$NETWORK" "--category=$CATEGORY"
}

# Check for help flag first
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    show_help
    exit 0
fi

# Run main function
main "$@"
