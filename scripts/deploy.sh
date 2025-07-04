#!/bin/bash

# Comprehensive deployment script for Pinocchio programs
# Usage: ./deploy.sh [program_name] [--network=devnet|testnet|mainnet]

set -e

# Default values
PROGRAM_NAME=${1:-"account_data"}
NETWORK="devnet"
CATEGORY="basics"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse command line arguments
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
    esac
done

# Set program directory based on category
PROGRAM_DIR="$CATEGORY/$PROGRAM_NAME"

# Help function
show_help() {
    echo "Usage: $0 [program_name] [--network=devnet|testnet|mainnet] [--category=basics]"
    echo ""
    echo "Comprehensive deployment script for Pinocchio programs"
    echo ""
    echo "Arguments:"
    echo "  program_name         Name of the program (default: account_data)"
    echo "  --network=NETWORK    Target network (default: devnet)"
    echo "  --category=CATEGORY  Program category (default: basics)"
    echo ""
    echo "Examples:"
    echo "  $0                           # Deploy account_data to devnet"
    echo "  $0 close_account             # Deploy close_account to devnet"
    echo "  $0 account_data --network=testnet  # Deploy to testnet"
    echo "  $0 token-basic --category=tokens   # Deploy tokens/token-basic"
    echo ""
    echo "The script will:"
    echo "  1. Set Solana cluster to the specified network"
    echo "  2. Build the program"
    echo "  3. Deploy the program"
    echo "  4. Update program IDs in source code"
    echo "  5. Regenerate TypeScript client"
}

# Function to check prerequisites
check_prerequisites() {
    echo -e "${BLUE}Checking prerequisites...${NC}"
    
    # Check if Solana CLI is installed
    if ! command -v solana &> /dev/null; then
        echo -e "${RED}Error: Solana CLI not found. Please install it first.${NC}"
        exit 1
    fi
    
    # Check if Cargo is installed
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: Cargo not found. Please install Rust first.${NC}"
        exit 1
    fi
    
    # Check if Bun is installed
    if ! command -v bun &> /dev/null; then
        echo -e "${RED}Error: Bun not found. Please install it first.${NC}"
        exit 1
    fi
    
    # Check if program directory exists
    if [ ! -d "$PROGRAM_DIR" ]; then
        echo -e "${RED}Error: Program directory $PROGRAM_DIR does not exist${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓ All prerequisites met${NC}"
}

# Function to set Solana cluster
set_solana_cluster() {
    echo -e "${BLUE}Setting Solana cluster to $NETWORK...${NC}"
    
    # Use RPC configuration utility to get the correct endpoint
    local rpc_endpoint
    
    case $NETWORK in
        devnet)
            rpc_endpoint=$(node scripts/rpc-config.js get devnet 2>/dev/null || echo "https://api.devnet.solana.com")
            ;;
        mainnet|mainnet-beta)
            rpc_endpoint=$(node scripts/rpc-config.js get mainnet 2>/dev/null || echo "https://api.mainnet-beta.solana.com")
            ;;
        testnet)
            rpc_endpoint=$(node scripts/rpc-config.js get testnet 2>/dev/null || echo "https://api.testnet.solana.com")
            ;;
        localhost|local)
            rpc_endpoint="http://localhost:8899"
            ;;
        *)
            echo -e "${RED}Error: Invalid network '$NETWORK'. Use devnet, testnet, mainnet, or localhost${NC}"
            exit 1
            ;;
    esac
    
    echo -e "${YELLOW}Using RPC endpoint: $rpc_endpoint${NC}"
    solana config set --url "$rpc_endpoint"
    
    echo -e "${GREEN}✓ Cluster set to $NETWORK${NC}"
}

# Function to check wallet balance
check_wallet_balance() {
    echo -e "${BLUE}Checking wallet balance...${NC}"
    
    local balance=$(solana balance --lamports 2>/dev/null || echo "0")
    local sol_balance
    
    # Simple calculation without bc dependency
    if [ "$balance" != "0" ] && [ -n "$balance" ]; then
        # Convert lamports to SOL using awk (more portable than bc)
        sol_balance=$(awk "BEGIN {printf \"%.9f\", $balance / 1000000000}")
    else
        sol_balance="0.000000000"
    fi
    
    echo -e "${GREEN}Wallet balance: $sol_balance SOL${NC}"
    
    # Check if balance is sufficient (at least 0.1 SOL for deployment)
    local balance_check=$(awk "BEGIN {print ($sol_balance < 0.1)}")
    
    if [ "$balance_check" = "1" ]; then
        echo -e "${YELLOW}Warning: Low wallet balance. You may need to fund your wallet.${NC}"
        if [[ "$NETWORK" == "devnet" || "$NETWORK" == "testnet" ]]; then
            echo -e "${YELLOW}You can get test SOL from: https://faucet.solana.com${NC}"
        fi
    fi
}

# Function to build the program
build_program() {
    echo -e "${BLUE}Building program $PROGRAM_NAME...${NC}"
    
    # Build for Solana BPF target
    cargo build-sbf --manifest-path "$PROGRAM_DIR/Cargo.toml"
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Program built successfully${NC}"
    else
        echo -e "${RED}Error: Failed to build program${NC}"
        exit 1
    fi
}

# Function to deploy the program
deploy_program() {
    echo -e "${BLUE}Deploying program $PROGRAM_NAME...${NC}"
    
    # Convert program name to use underscores for binary paths (Cargo uses underscores)
    local program_name_underscore=$(echo "$PROGRAM_NAME" | tr '-' '_')
    local binary_path="target/deploy/$program_name_underscore.so"
    local keypair_path="target/deploy/$program_name_underscore-keypair.json"
    
    if [ ! -f "$binary_path" ]; then
        echo -e "${RED}Error: Program binary not found at $binary_path${NC}"
        exit 1
    fi
    
    # Deploy the program
    if [ -f "$keypair_path" ]; then
        echo -e "${YELLOW}Using existing keypair for deployment...${NC}"
        solana program deploy "$binary_path" --program-id "$keypair_path"
    else
        echo -e "${YELLOW}Creating new program keypair...${NC}"
        solana program deploy "$binary_path" --keypair "$keypair_path"
    fi
    
    if [ $? -eq 0 ]; then
        local program_id=$(solana-keygen pubkey "$keypair_path")
        echo -e "${GREEN}✓ Program deployed successfully${NC}"
        echo -e "${GREEN}Program ID: $program_id${NC}"
        return 0
    else
        echo -e "${RED}Error: Failed to deploy program${NC}"
        exit 1
    fi
}

# Function to update program IDs using the update script
update_program_ids() {
    echo -e "${BLUE}Updating program IDs in source code...${NC}"
    
    if [ -f "./scripts/update-program-ids.sh" ]; then
        chmod +x "./scripts/update-program-ids.sh"
        "./scripts/update-program-ids.sh" "$PROGRAM_NAME" --category="$CATEGORY"
    else
        echo -e "${YELLOW}Warning: update-program-ids.sh script not found${NC}"
        echo -e "${YELLOW}Manually regenerating IDL and client...${NC}"
        
        # Fallback: generate IDL and client directly
        npm run "gen:idl:$PROGRAM_NAME" 2>/dev/null || echo "IDL generation failed"
        npm run "gen:client:$PROGRAM_NAME" 2>/dev/null || echo "Client generation failed"
    fi
}

# Function to run tests
run_tests() {
    echo -e "${BLUE}Running tests...${NC}"
    
    local test_script="test:client:$(echo $PROGRAM_NAME | tr '_' '-')"
    
    if grep -q "\"$test_script\"" package.json; then
        bun run "$test_script"
        if [ $? -eq 0 ]; then
            echo -e "${GREEN}✓ Tests passed${NC}"
        else
            echo -e "${YELLOW}Warning: Some tests failed${NC}"
        fi
    else
        echo -e "${YELLOW}No test script found for $PROGRAM_NAME${NC}"
    fi
}

# Function to display deployment summary
show_summary() {
    local program_name_underscore=$(echo "$PROGRAM_NAME" | tr '-' '_')
    local binary_path="target/deploy/$program_name_underscore.so"
    local keypair_path="target/deploy/$program_name_underscore-keypair.json"
    
    if [ -f "$keypair_path" ]; then
        local program_id=$(solana-keygen pubkey "$keypair_path")
        local current_cluster=$(solana config get | grep "RPC URL" | awk '{print $3}')
        
        echo ""
        echo -e "${GREEN}🎉 Deployment Summary${NC}"
        echo -e "${GREEN}=====================${NC}"
        echo -e "${GREEN}Program Name: $PROGRAM_NAME${NC}"
        echo -e "${GREEN}Program ID: $program_id${NC}"
        echo -e "${GREEN}Network: $current_cluster${NC}"
        echo -e "${GREEN}Binary: $binary_path${NC}"
        echo -e "${GREEN}Keypair: $keypair_path${NC}"
        echo ""
        echo -e "${YELLOW}Next steps:${NC}"
        echo -e "${YELLOW}- Commit your changes: git add . && git commit -m 'Deploy $PROGRAM_NAME'${NC}"
        echo -e "${YELLOW}- Test your program with the updated client${NC}"
    fi
}

# Main execution
main() {
    echo -e "${GREEN}🚀 Starting deployment for $PROGRAM_NAME${NC}"
    echo ""
    
    check_prerequisites
    set_solana_cluster
    check_wallet_balance
    build_program
    deploy_program
    update_program_ids
    run_tests
    show_summary
    
    echo -e "${GREEN}✅ Deployment completed successfully!${NC}"
}

# Check for help flag
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    show_help
    exit 0
fi

# Run main function
main
