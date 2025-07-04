#!/bin/bash

# Script to update program IDs after deployment
# Usage: ./update-program-ids.sh [program_name]

set -e

PROGRAM_NAME=${1:-"account_data"}
CATEGORY="basics"
CLIENT_DIR="clients"

# Parse command line arguments
for arg in "$@"; do
    case $arg in
        --category=*)
            CATEGORY="${arg#*=}"
            ;;
    esac
done

# Set program directory based on category
PROGRAM_DIR="$CATEGORY/$PROGRAM_NAME"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Updating program IDs for $PROGRAM_NAME...${NC}"

# Check if program directory exists
if [ ! -d "$PROGRAM_DIR" ]; then
    echo -e "${RED}Error: Program directory $PROGRAM_DIR does not exist${NC}"
    exit 1
fi

# Function to extract program ID from keypair file
get_program_id_from_keypair() {
    local keypair_file="$1"
    if [ -f "$keypair_file" ]; then
        solana-keygen pubkey "$keypair_file"
    else
        echo ""
    fi
}

# Function to get program ID from deployed program
get_deployed_program_id() {
    local program_path="$1"
    # Try to get program ID from the binary metadata
    # If binary exists, we can extract the program ID
    if [ -f "$program_path" ]; then
        # Extract program ID from the deployed binary using solana program show
        # This requires the program to be deployed first
        local temp_keypair=$(mktemp)
        solana-keygen new --no-bip39-passphrase --silent --outfile "$temp_keypair"
        local result=$(solana program show --programs 2>/dev/null | grep "$(basename $program_path .so)" | awk '{print $1}' || echo "")
        rm -f "$temp_keypair"
        echo "$result"
    else
        echo ""
    fi
}

# Function to update Rust lib.rs file
update_rust_program_id() {
    local new_program_id="$1"
    local lib_file="$PROGRAM_DIR/src/lib.rs"
    
    if [ -f "$lib_file" ]; then
        echo -e "${YELLOW}Updating Rust program ID in $lib_file${NC}"
        
        # Create backup
        cp "$lib_file" "$lib_file.backup"
        
        # Update the declare_id! and shank attribute
        sed -i "s/pinocchio_pubkey::declare_id!(\"[^\"]*\")/pinocchio_pubkey::declare_id!(\"$new_program_id\")/g" "$lib_file"
        sed -i "s/#\[shank(id = \"[^\"]*\")\]/#[shank(id = \"$new_program_id\")]/g" "$lib_file"
        
        echo -e "${GREEN}✓ Updated Rust program ID${NC}"
    else
        echo -e "${RED}Warning: Rust lib.rs file not found at $lib_file${NC}"
    fi
}

# Function to update test files with new program ID
update_test_program_id() {
    local new_program_id="$1"
    local test_dir="$PROGRAM_DIR/tests"
    
    if [ -d "$test_dir" ]; then
        echo -e "${YELLOW}Updating program ID in test files...${NC}"
        
        # Find all .ts test files and update program ID
        find "$test_dir" -name "*.ts" -type f | while read -r test_file; do
            # Handle both Kite address format and legacy PublicKey format
            if grep -q "address('11111111111111111111111111111111')" "$test_file"; then
                sed -i "s/address('11111111111111111111111111111111')/address('$new_program_id')/g" "$test_file"
                echo -e "${GREEN}✓ Updated Kite program ID in $(basename $test_file)${NC}"
            elif grep -q "new PublicKey('11111111111111111111111111111111')" "$test_file"; then
                sed -i "s/new PublicKey('11111111111111111111111111111111')/new PublicKey('$new_program_id')/g" "$test_file"
                echo -e "${GREEN}✓ Updated legacy program ID in $(basename $test_file)${NC}"
            fi
        done
    fi
}

# Function to regenerate IDL and client using Shank approach
regenerate_idl_and_client() {
    echo -e "${YELLOW}Regenerating IDL from Rust code using Shank...${NC}"
    
    # Generate IDL using Shank
    local program_name_snake=$(echo $PROGRAM_NAME | tr '-' '_')
    shank idl --crate-root "$PROGRAM_DIR" --out-dir idl
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ IDL generated successfully${NC}"
        
        # Generate TypeScript client using Codama
        echo -e "${YELLOW}Generating TypeScript client from IDL...${NC}"
        node scripts/generate-clients.js "$PROGRAM_NAME"
        
        if [ $? -eq 0 ]; then
            echo -e "${GREEN}✓ TypeScript client generated successfully${NC}"
        else
            echo -e "${RED}Warning: Client generation failed${NC}"
        fi
    else
        echo -e "${RED}Warning: IDL generation failed${NC}"
    fi
}

# Function to regenerate client code
regenerate_client() {
    echo -e "${YELLOW}Regenerating TypeScript client...${NC}"
    
    if [ -f "$PROGRAM_DIR/generate-client.ts" ]; then
        cd "$PROGRAM_DIR"
        bun generate-client.ts
        cd - > /dev/null
        echo -e "${GREEN}✓ Client code regenerated${NC}"
    else
        echo -e "${RED}Warning: generate-client.ts not found${NC}"
    fi
}

# Main logic
main() {
    local program_id=""
    local program_name_underscore=$(echo "$PROGRAM_NAME" | tr '-' '_')
    local keypair_file="target/deploy/$program_name_underscore-keypair.json"
    local binary_file="target/deploy/$program_name_underscore.so"
    
    # Try different methods to get the program ID
    if [ -f "$keypair_file" ]; then
        program_id=$(get_program_id_from_keypair "$keypair_file")
        echo -e "${GREEN}Found program ID from keypair: $program_id${NC}"
    elif [ -f "$binary_file" ]; then
        program_id=$(get_deployed_program_id "$binary_file")
        if [ -n "$program_id" ]; then
            echo -e "${GREEN}Found program ID from deployed program: $program_id${NC}"
        fi
    fi
    
    # If no program ID found, prompt user
    if [ -z "$program_id" ]; then
        echo -e "${YELLOW}No program ID found automatically.${NC}"
        echo -e "${YELLOW}Please enter the program ID manually:${NC}"
        read -p "Program ID: " program_id
        
        if [ -z "$program_id" ]; then
            echo -e "${RED}Error: No program ID provided${NC}"
            exit 1
        fi
    fi
    
    # Validate program ID format (should be base58 and ~44 characters)
    if [[ ! "$program_id" =~ ^[1-9A-HJ-NP-Za-km-z]{32,44}$ ]]; then
        echo -e "${RED}Error: Invalid program ID format: $program_id${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}Using program ID: $program_id${NC}"
    
    # Update all files
    update_rust_program_id "$program_id"
    update_test_program_id "$program_id"
    regenerate_idl_and_client
    
    echo -e "${GREEN}✓ Program ID update complete!${NC}"
    echo -e "${YELLOW}Don't forget to commit your changes.${NC}"
}

# Help function
show_help() {
    echo "Usage: $0 [program_name]"
    echo ""
    echo "Updates program IDs after deployment for a Pinocchio program"
    echo ""
    echo "Arguments:"
    echo "  program_name    Name of the program (default: account_data)"
    echo ""
    echo "Examples:"
    echo "  $0                    # Update account_data program"
    echo "  $0 close_account      # Update close_account program"
    echo ""
    echo "The script will:"
    echo "  1. Try to find the program ID from keypair or deployed program"
    echo "  2. Update the Rust lib.rs file"
    echo "  3. Update the Codama configuration"
    echo "  4. Regenerate the TypeScript client"
}

# Check for help flag
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    show_help
    exit 0
fi

# Check if required tools are available
if ! command -v solana-keygen &> /dev/null; then
    echo -e "${RED}Error: solana-keygen not found. Please install Solana CLI tools.${NC}"
    exit 1
fi

if ! command -v bun &> /dev/null; then
    echo -e "${RED}Error: bun not found. Please install Bun.${NC}"
    exit 1
fi

# Run main function
main
