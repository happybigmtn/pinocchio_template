#!/bin/bash

# Quick deployment script that creates, builds, and deploys a new program
# Usage: ./quick-deploy.sh [program_name] [--category=basics] [--network=devnet]

set -e

# Default values
PROGRAM_NAME="$1"
CATEGORY="basics"
NETWORK="devnet"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse command line arguments
for arg in "$@"; do
    case $arg in
        --category=*)
            CATEGORY="${arg#*=}"
            ;;
        --network=*)
            NETWORK="${arg#*=}"
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
    esac
done

# Help function
show_help() {
    echo "Usage: $0 [program_name] [--category=basics] [--network=devnet]"
    echo ""
    echo "Creates and deploys a new Pinocchio program in one command"
    echo ""
    echo "Arguments:"
    echo "  program_name         Name of the new program (required)"
    echo "  --category=CATEGORY  Category for the program (default: basics)"
    echo "  --network=NETWORK    Target network (default: devnet)"
    echo ""
    echo "Examples:"
    echo "  $0 my_counter                           # Create and deploy basics/my_counter to devnet"
    echo "  $0 token_mint --category=tokens         # Create and deploy tokens/token_mint"
    echo "  $0 my_program --network=testnet         # Deploy to testnet"
    echo ""
    echo "This script will:"
    echo "  1. Create the program from template"
    echo "  2. Build the program"
    echo "  3. Deploy to the specified network"
    echo "  4. Update program IDs"
    echo "  5. Generate TypeScript client"
    echo "  6. Run tests"
}

# Validate inputs
validate_inputs() {
    if [ -z "$PROGRAM_NAME" ]; then
        echo -e "${RED}Error: Program name is required${NC}"
        show_help
        exit 1
    fi

    # Check if program already exists
    if [ -d "$CATEGORY/$PROGRAM_NAME" ]; then
        echo -e "${RED}Error: Program $CATEGORY/$PROGRAM_NAME already exists${NC}"
        exit 1
    fi
}

# Main execution
main() {
    echo -e "${GREEN}🚀 Quick Deploy: Creating and deploying $PROGRAM_NAME${NC}"
    echo ""
    
    validate_inputs
    
    # Step 1: Create the program
    echo -e "${BLUE}Step 1: Creating program from template...${NC}"
    ./create-program.sh "$PROGRAM_NAME" --category="$CATEGORY"
    
    if [ $? -ne 0 ]; then
        echo -e "${RED}Error: Failed to create program${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓ Program created successfully${NC}"
    echo ""
    
    # Step 2: Deploy the program
    echo -e "${BLUE}Step 2: Deploying program...${NC}"
    ./deploy.sh "$PROGRAM_NAME" --network="$NETWORK"
    
    if [ $? -ne 0 ]; then
        echo -e "${RED}Error: Failed to deploy program${NC}"
        exit 1
    fi
    
    echo ""
    echo -e "${GREEN}🎉 Quick Deploy Summary${NC}"
    echo -e "${GREEN}======================${NC}"
    echo -e "${GREEN}Program: $PROGRAM_NAME${NC}"
    echo -e "${GREEN}Category: $CATEGORY${NC}"
    echo -e "${GREEN}Network: $NETWORK${NC}"
    echo -e "${GREEN}Location: $CATEGORY/$PROGRAM_NAME${NC}"
    echo ""
    echo -e "${YELLOW}Your program is now ready for development!${NC}"
    echo -e "${YELLOW}Next steps:${NC}"
    echo -e "${YELLOW}- Edit the code in $CATEGORY/$PROGRAM_NAME/src/${NC}"
    echo -e "${YELLOW}- Test with: npm run test:client:$(echo $PROGRAM_NAME | tr '_' '-')${NC}"
    echo -e "${YELLOW}- Redeploy after changes: ./deploy.sh $PROGRAM_NAME${NC}"
}

# Check for help flag
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    show_help
    exit 0
fi

# Run main function
main
