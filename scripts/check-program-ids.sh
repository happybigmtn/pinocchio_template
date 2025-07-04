#!/bin/bash

# Script to check current program ID status
# Usage: ./check-program-ids.sh [program_name]

PROGRAM_NAME=${1:-"account_data"}
PROGRAM_DIR="basics/$PROGRAM_NAME"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Checking program ID status for $PROGRAM_NAME...${NC}"
echo ""

# Check if program directory exists
if [ ! -d "$PROGRAM_DIR" ]; then
    echo -e "${RED}Error: Program directory $PROGRAM_DIR does not exist${NC}"
    exit 1
fi

# Function to extract program ID from file
get_program_id_from_rust() {
    local lib_file="$PROGRAM_DIR/src/lib.rs"
    if [ -f "$lib_file" ]; then
        grep 'declare_id!' "$lib_file" | sed 's/.*declare_id!("\([^"]*\)").*/\1/'
    fi
}

get_program_id_from_codama() {
    local codama_file="$PROGRAM_DIR/codama-node.ts"
    if [ -f "$codama_file" ]; then
        grep "publicKey:" "$codama_file" | sed "s/.*publicKey: '\([^']*\)'.*/\1/"
    fi
}

get_program_id_from_client() {
    # Find program file specific to our program (look for ACCOUNT_DATA_PROGRAM_ADDRESS or similar)
    local program_name_upper=$(echo $PROGRAM_NAME | tr '[:lower:]' '[:upper:]' | tr '_-' '__')
    local search_pattern="${program_name_upper}_PROGRAM_ADDRESS"
    
    local client_files=$(find clients -name "*.ts" -exec grep -l "$search_pattern" {} \; 2>/dev/null | head -1)
    
    if [ -n "$client_files" ]; then
        # Get the line after the specific PROGRAM_ADDRESS and extract the ID
        grep -A1 "$search_pattern" "$client_files" | tail -1 | sed "s/.*'\([A-Za-z0-9]*\)'.*/\1/"
    fi
}

get_program_id_from_keypair() {
    local keypair_file="target/deploy/$PROGRAM_NAME-keypair.json"
    if [ -f "$keypair_file" ]; then
        solana-keygen pubkey "$keypair_file" 2>/dev/null
    fi
}

# Get program IDs from different sources
RUST_ID=$(get_program_id_from_rust)
CODAMA_ID=$(get_program_id_from_codama)
CLIENT_ID=$(get_program_id_from_client)
DEPLOYED_ID=$(get_program_id_from_keypair)

echo -e "${YELLOW}Program ID Sources:${NC}"
echo "┌─────────────────┬──────────────────────────────────────────────┐"
echo "│ Source          │ Program ID                                   │"
echo "├─────────────────┼──────────────────────────────────────────────┤"
printf "│ %-15s │ %-44s │\n" "Rust (lib.rs)" "${RUST_ID:-"Not found"}"
printf "│ %-15s │ %-44s │\n" "Codama Config" "${CODAMA_ID:-"Not found"}"
printf "│ %-15s │ %-44s │\n" "TS Client" "${CLIENT_ID:-"Not found"}"
printf "│ %-15s │ %-44s │\n" "Deployed" "${DEPLOYED_ID:-"Not deployed"}"
echo "└─────────────────┴──────────────────────────────────────────────┘"
echo ""

# Check consistency
ALL_IDS=()
[ -n "$RUST_ID" ] && ALL_IDS+=("$RUST_ID")
[ -n "$CODAMA_ID" ] && ALL_IDS+=("$CODAMA_ID")
[ -n "$CLIENT_ID" ] && ALL_IDS+=("$CLIENT_ID")

if [ ${#ALL_IDS[@]} -eq 0 ]; then
    echo -e "${RED}❌ No program IDs found${NC}"
    exit 1
fi

# Check if all IDs are the same
FIRST_ID="${ALL_IDS[0]}"
ALL_SAME=true
for id in "${ALL_IDS[@]}"; do
    if [ "$id" != "$FIRST_ID" ]; then
        ALL_SAME=false
        break
    fi
done

if [ "$ALL_SAME" = true ]; then
    echo -e "${GREEN}✅ All source code program IDs are consistent${NC}"
    
    # Check if deployed matches source
    if [ -n "$DEPLOYED_ID" ]; then
        if [ "$DEPLOYED_ID" = "$FIRST_ID" ]; then
            echo -e "${GREEN}✅ Deployed program ID matches source code${NC}"
            echo -e "${GREEN}🎉 Everything is in sync!${NC}"
        else
            echo -e "${YELLOW}⚠️  Deployed program ID differs from source code${NC}"
            echo -e "${YELLOW}   Run: ./update-program-ids.sh $PROGRAM_NAME${NC}"
        fi
    else
        echo -e "${YELLOW}ℹ️  No deployed program found${NC}"
        echo -e "${YELLOW}   Run: ./deploy.sh $PROGRAM_NAME${NC}"
    fi
else
    echo -e "${RED}❌ Program IDs are inconsistent across sources${NC}"
    echo -e "${YELLOW}   Run: ./update-program-ids.sh $PROGRAM_NAME${NC}"
fi

echo ""
echo -e "${BLUE}Available commands:${NC}"
echo -e "  ${YELLOW}./check-program-ids.sh $PROGRAM_NAME${NC} - Check status"
echo -e "  ${YELLOW}./update-program-ids.sh $PROGRAM_NAME${NC} - Update IDs from deployed program"
echo -e "  ${YELLOW}./deploy.sh $PROGRAM_NAME${NC} - Full deployment pipeline"
