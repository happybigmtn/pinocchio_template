#!/bin/bash

# Smart generate script for Pinocchio programs
# Automatically detects program name from current directory and generates IDL + client
# Usage: ./smart-generate.sh

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the current directory (use ORIGINAL_PWD if available, fallback to PWD)
# When called from bun, the working directory changes to package root, so we need to preserve the original
if [ -n "$ORIGINAL_PWD" ]; then
    CURRENT_DIR="$ORIGINAL_PWD"
else
    # Check if we have the original working directory from environment
    if [ -n "$INIT_CWD" ]; then
        CURRENT_DIR="$INIT_CWD"
    else
        CURRENT_DIR="$PWD"
    fi
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Function to detect program name and category from current directory
detect_program_info() {
    local rel_path=$(realpath --relative-to="$ROOT_DIR" "$CURRENT_DIR" 2>/dev/null || echo "$CURRENT_DIR")
    
    
    # Check if we're in a program directory (e.g., basics/account_data, tokens/test-token)
    if [[ "$rel_path" =~ ^([^/]+)/([^/]+)$ ]]; then
        CATEGORY="${BASH_REMATCH[1]}"
        PROGRAM_NAME="${BASH_REMATCH[2]}"
        echo -e "${GREEN}Detected program: $CATEGORY/$PROGRAM_NAME${NC}"
        return 0
    fi
    
    # Check if we're in the root directory and need to specify program
    if [[ "$rel_path" == "." || "$rel_path" == "" ]]; then
        echo -e "${RED}Error: Please run this command from a program directory (e.g., basics/account_data)${NC}"
        echo -e "${YELLOW}Or specify the program name: bun generate [program_name] --category=[category]${NC}"
        exit 1
    fi
    
    return 1
}

# Function to generate IDL
generate_idl() {
    echo -e "${BLUE}Generating IDL for $PROGRAM_NAME...${NC}"
    
    # Use the actual directory name (which should match the program structure)
    local crate_root="$CATEGORY/$PROGRAM_NAME"
    
    # Check if the crate root exists
    if [ ! -d "$ROOT_DIR/$crate_root" ]; then
        echo -e "${RED}Error: Program directory not found at $crate_root${NC}"
        exit 1
    fi
    
    # Verify Cargo.toml exists
    if [ ! -f "$ROOT_DIR/$crate_root/Cargo.toml" ]; then
        echo -e "${RED}Error: Cargo.toml not found in $crate_root${NC}"
        exit 1
    fi
    
    cd "$ROOT_DIR"
    
    # Create IDL directory if it doesn't exist
    mkdir -p idl
    
    # Generate IDL using shank
    echo -e "${BLUE}Running: shank idl --crate-root $crate_root --out-dir idl${NC}"
    shank idl --crate-root "$crate_root" --out-dir idl
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ IDL generated successfully${NC}"
    else
        echo -e "${RED}Error: Failed to generate IDL${NC}"
        exit 1
    fi
}

# Function to generate TypeScript client
generate_client() {
    echo -e "${BLUE}Generating TypeScript client for $PROGRAM_NAME...${NC}"
    
    cd "$ROOT_DIR"
    
    # Generate client using codama
    node scripts/generate-clients.js "$PROGRAM_NAME"
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ TypeScript client generated successfully${NC}"
    else
        echo -e "${RED}Error: Failed to generate TypeScript client${NC}"
        exit 1
    fi
}

# Function to show summary
show_summary() {
    echo ""
    echo -e "${GREEN}🎉 Generation Complete${NC}"
    echo -e "${GREEN}=====================${NC}"
    echo -e "${GREEN}Program: $CATEGORY/$PROGRAM_NAME${NC}"
    echo -e "${GREEN}IDL: idl/$(echo $PROGRAM_NAME | tr '-' '_').json${NC}"
    
    # Find the client directory (the generate-clients.js shows the location)
    echo -e "${GREEN}Client: Check output above for client location${NC}"
    
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo -e "${YELLOW}- Review the generated IDL and client${NC}"
    echo -e "${YELLOW}- Run tests: bun test${NC}"
    echo -e "${YELLOW}- Deploy program: bun run deploy${NC}"
}

# Main execution
main() {
    echo -e "${GREEN}🔧 Smart Generate for Pinocchio Programs${NC}"
    echo ""
    
    # Parse command line arguments for explicit program name
    if [ -n "$1" ]; then
        PROGRAM_NAME="$1"
        CATEGORY="basics"  # Default category
        
        # Parse category flag
        for arg in "$@"; do
            case $arg in
                --category=*)
                    CATEGORY="${arg#*=}"
                    ;;
            esac
        done
        
        echo -e "${GREEN}Using specified program: $CATEGORY/$PROGRAM_NAME${NC}"
    else
        # Auto-detect from directory
        detect_program_info
    fi
    
    generate_idl
    generate_client
    show_summary
    
    echo -e "${GREEN}✅ Generation completed successfully!${NC}"
}

# Check for help flag
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    echo "Usage: $0 [program_name] [--category=basics|tokens|compression|oracles]"
    echo ""
    echo "Smart generate script for Pinocchio programs"
    echo ""
    echo "When run from a program directory (e.g., basics/account_data):"
    echo "  $0                    # Auto-detects program and generates IDL + client"
    echo ""
    echo "When run with explicit arguments:"
    echo "  $0 my_program                    # Generate for basics/my_program"
    echo "  $0 token_mint --category=tokens  # Generate for tokens/token_mint"
    echo ""
    echo "The script will:"
    echo "  1. Generate IDL using shank"
    echo "  2. Generate TypeScript client using codama"
    echo "  3. Show generation summary"
    exit 0
fi

# Run main function
main "$@"
