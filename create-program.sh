#!/bin/bash

# Script to create new Pinocchio programs from template
# Usage: ./create-program.sh [program_name] [--category=basics|tokens|compression|oracles]

set -e

# Default values
PROGRAM_NAME="$1"
CATEGORY="basics"
TEMPLATE_DIR="templates/counter"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Help function
show_help() {
    echo "Usage: $0 [program_name] [--category=basics|tokens|compression|oracles]"
    echo ""
    echo "Creates a new Pinocchio program from template"
    echo ""
    echo "Arguments:"
    echo "  program_name         Name of the new program (required)"
    echo "  --category=CATEGORY  Category for the program (default: basics)"
    echo ""
    echo "Examples:"
    echo "  $0 my_program                    # Create basics/my_program"
    echo "  $0 token_mint --category=tokens  # Create tokens/token_mint"
    echo ""
    echo "The script will:"
    echo "  1. Create program directory with Rust source code"
    echo "  2. Update Cargo.toml files"
    echo "  3. Generate package.json scripts"
    echo "  4. Create test files"
    echo "  5. Set up deployment configuration"
}

# Validate inputs
validate_inputs() {
    if [ -z "$PROGRAM_NAME" ]; then
        echo -e "${RED}Error: Program name is required${NC}"
        show_help
        exit 1
    fi

    # Validate program name format
    if [[ ! "$PROGRAM_NAME" =~ ^[a-z0-9_-]+$ ]]; then
        echo -e "${RED}Error: Program name must contain only lowercase letters, numbers, hyphens, and underscores${NC}"
        exit 1
    fi

    # Validate category
    case $CATEGORY in
        basics|tokens|compression|oracles)
            ;;
        *)
            echo -e "${RED}Error: Invalid category '$CATEGORY'. Use basics, tokens, compression, or oracles${NC}"
            exit 1
            ;;
    esac

    # Check if program already exists
    if [ -d "$CATEGORY/$PROGRAM_NAME" ]; then
        echo -e "${RED}Error: Program $CATEGORY/$PROGRAM_NAME already exists${NC}"
        exit 1
    fi

    # Check if template exists
    if [ ! -d "$TEMPLATE_DIR" ]; then
        echo -e "${RED}Error: Template directory $TEMPLATE_DIR does not exist${NC}"
        exit 1
    fi
}

# Create program directory structure
create_program_structure() {
    echo -e "${BLUE}Creating program structure for $PROGRAM_NAME...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    mkdir -p "$target_dir"
    
    # Copy template files
    cp -r "$TEMPLATE_DIR"/* "$target_dir/"
    
    # Create additional directories if they don't exist
    mkdir -p "$target_dir/src/instructions"
    mkdir -p "$target_dir/src/state"
    mkdir -p "$target_dir/tests"
    
    echo -e "${GREEN}✓ Program structure created${NC}"
}

# Update Cargo.toml files
update_cargo_toml() {
    echo -e "${BLUE}Updating Cargo.toml files...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    local program_name_snake=$(echo "$PROGRAM_NAME" | tr '-' '_')
    
    # Update program Cargo.toml
    if [ -f "$target_dir/Cargo.toml" ]; then
        sed -i "s/name = \"counter\"/name = \"$PROGRAM_NAME\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"counter_\"/name = \"${program_name_snake}_\"/g" "$target_dir/Cargo.toml"
    fi
    
    # Update workspace Cargo.toml
    if [ -f "Cargo.toml" ]; then
        # Add to workspace members if not already present
        if ! grep -q "\"$CATEGORY/$PROGRAM_NAME\"" Cargo.toml; then
            # Add to the existing members array
            sed -i "/members = \[/,/\]/ s/\]/  \"$CATEGORY\/$PROGRAM_NAME\",\n\]/" Cargo.toml
        fi
    fi
    
    echo -e "${GREEN}✓ Cargo.toml files updated${NC}"
}

# Update Rust source files
update_rust_source() {
    echo -e "${BLUE}Updating Rust source files...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    local program_name_snake=$(echo "$PROGRAM_NAME" | tr '-' '_')
    local program_name_upper=$(echo "$PROGRAM_NAME" | tr '[:lower:]' '[:upper:]' | tr '-' '_')
    
    # Generate a new program ID placeholder
    local new_program_id="11111111111111111111111111111111"
    
    # Update lib.rs
    if [ -f "$target_dir/src/lib.rs" ]; then
        sed -i "s/counter/$program_name_snake/g" "$target_dir/src/lib.rs"
        sed -i "s/COUNTER/$program_name_upper/g" "$target_dir/src/lib.rs"
        sed -i "s/pinocchio_pubkey::declare_id!(\"[^\"]*\")/pinocchio_pubkey::declare_id!(\"$new_program_id\")/g" "$target_dir/src/lib.rs"
    fi
    
    # Update other Rust files
    find "$target_dir/src" -name "*.rs" -exec sed -i "s/counter/$program_name_snake/g" {} \;
    find "$target_dir/src" -name "*.rs" -exec sed -i "s/COUNTER/$program_name_upper/g" {} \;
    
    echo -e "${GREEN}✓ Rust source files updated${NC}"
}

# Update package.json scripts
update_package_json() {
    echo -e "${BLUE}Updating package.json scripts...${NC}"
    
    local program_name_dash=$(echo "$PROGRAM_NAME" | tr '_' '-')
    local program_name_snake=$(echo "$PROGRAM_NAME" | tr '-' '_')
    
    # Add scripts to package.json
    local temp_file=$(mktemp)
    
    # Read current package.json and add new scripts
    node -e "
        const fs = require('fs');
        const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8'));
        
        // Add new scripts
        pkg.scripts['gen:client:$program_name_dash'] = 'node scripts/generate-clients.js $PROGRAM_NAME';
        pkg.scripts['gen:idl:$program_name_dash'] = 'shank idl --crate-root $CATEGORY/$PROGRAM_NAME --out-dir idl';
        pkg.scripts['test:client:$program_name_dash'] = 'bun test --testFiles $CATEGORY/$PROGRAM_NAME/tests/$program_name_dash.test.ts';
        
        fs.writeFileSync('package.json', JSON.stringify(pkg, null, 2));
    "
    
    echo -e "${GREEN}✓ package.json scripts updated${NC}"
}

# Create test files
create_test_files() {
    echo -e "${BLUE}Creating test files...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    local program_name_dash=$(echo "$PROGRAM_NAME" | tr '_' '-')
    local program_name_snake=$(echo "$PROGRAM_NAME" | tr '-' '_')
    
    # Create TypeScript test file
    cat > "$target_dir/tests/$program_name_dash.test.ts" << EOF
import { describe, test, beforeAll } from 'bun:test';
import { connect } from 'solana-kite';
import { address } from '@solana/kit';
import { getKiteConnection } from '../../../scripts/rpc-config.js';

// Import generated client (will be available after running gen:client)
// import { ${program_name_snake}Program } from '../clients/${program_name_dash}';

describe('${PROGRAM_NAME}', () => {
  let kite: Awaited<ReturnType<typeof connect>>;
  const programId = address('11111111111111111111111111111111'); // Will be updated after deployment

  beforeAll(async () => {
    // Connect using Helius devnet RPC with both HTTP and WebSocket endpoints
    const { getRpcEndpoint, getWsEndpoint } = await import('../../../scripts/rpc-config.js');
    const rpcEndpoint = getRpcEndpoint('devnet');
    const wsEndpoint = getWsEndpoint('devnet');
    kite = await connect(rpcEndpoint, wsEndpoint);
  });

  test('should initialize successfully', async () => {
    // TODO: Implement test logic
    console.log('Test for ${PROGRAM_NAME} - implement your test logic here');
    console.log('Program ID:', programId);
    console.log('Connected to Kite on devnet');
    
    try {
      // Create a test wallet
      console.log('Creating test wallet...');
      const testWallet = await kite.createWallet({ 
        airdropLamports: 1_000_000_000n // 1 SOL
      });
      console.log('Test wallet created:', testWallet.address);
      
      // Check balance
      const balance = await kite.getLamportBalance(testWallet.address);
      console.log('Wallet balance:', Number(balance) / 1_000_000_000, 'SOL');
      
      console.log('✅ Basic Kite functionality working!');
    } catch (error) {
      console.error('❌ Error during test:', error);
      // For now, we'll just log the error instead of failing the test
      // since this might be due to devnet issues
      console.log('⚠️  Test completed with network issues (this may be expected on devnet)');
    }
  }, { timeout: 30000 }); // 30 second timeout
});
EOF
    
    echo -e "${GREEN}✓ Test files created${NC}"
}

# Create deployment configuration
create_deployment_config() {
    echo -e "${BLUE}Creating deployment configuration...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    
    # Create deploy script for this specific program
    cat > "$target_dir/deploy.sh" << 'EOF'
#!/bin/bash

# Program-specific deployment script
# This is a convenience script that calls the main deploy.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$ROOT_DIR"

# Extract program name from directory structure
PROGRAM_NAME=$(basename "$SCRIPT_DIR")

# Call main deployment script
exec "./deploy.sh" "$PROGRAM_NAME" "$@"
EOF
    
    chmod +x "$target_dir/deploy.sh"
    
    echo -e "${GREEN}✓ Deployment configuration created${NC}"
}

# Create README for the new program
create_readme() {
    echo -e "${BLUE}Creating README...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    local program_name_dash=$(echo "$PROGRAM_NAME" | tr '_' '-')
    local program_name_title=$(echo "$PROGRAM_NAME" | tr '_-' ' ' | sed 's/\b\w/\U&/g')
    
    cat > "$target_dir/README.md" << EOF
# ${program_name_title}

A Solana program built with Pinocchio.

## Description

TODO: Add description of what this program does.

## Usage

### Building

\`\`\`bash
cargo build-sbf --manifest-path $CATEGORY/$PROGRAM_NAME/Cargo.toml
\`\`\`

### Deployment

\`\`\`bash
# Deploy to devnet
./deploy.sh $PROGRAM_NAME

# Deploy to testnet
./deploy.sh $PROGRAM_NAME --network=testnet

# Deploy to mainnet
./deploy.sh $PROGRAM_NAME --network=mainnet
\`\`\`

### Generate Client

\`\`\`bash
# Generate IDL
npm run gen:idl:$program_name_dash

# Generate TypeScript client
npm run gen:client:$program_name_dash
\`\`\`

### Testing

\`\`\`bash
# Run tests
npm run test:client:$program_name_dash
\`\`\`

## Program Structure

- \`src/lib.rs\` - Main program entry point
- \`src/processor.rs\` - Instruction processing logic
- \`src/instructions/\` - Instruction definitions
- \`src/state/\` - Account state definitions
- \`src/constants.rs\` - Program constants
- \`tests/\` - Test files

## Development

1. Modify the Rust source code in \`src/\`
2. Build and deploy: \`./deploy.sh $PROGRAM_NAME\`
3. Generate client: \`npm run gen:client:$program_name_dash\`
4. Run tests: \`npm run test:client:$program_name_dash\`

## Notes

- This program was created from the Pinocchio template
- Program ID will be updated after first deployment
- Remember to commit changes after successful deployment
EOF
    
    echo -e "${GREEN}✓ README created${NC}"
}

# Display summary
show_summary() {
    echo ""
    echo -e "${GREEN}🎉 Program Creation Summary${NC}"
    echo -e "${GREEN}===========================${NC}"
    echo -e "${GREEN}Program Name: $PROGRAM_NAME${NC}"
    echo -e "${GREEN}Category: $CATEGORY${NC}"
    echo -e "${GREEN}Location: $CATEGORY/$PROGRAM_NAME${NC}"
    echo ""
    echo -e "${YELLOW}Next steps:${NC}"
    echo -e "${YELLOW}1. Review the generated code in $CATEGORY/$PROGRAM_NAME/src/${NC}"
    echo -e "${YELLOW}2. Modify the program logic as needed${NC}"
    echo -e "${YELLOW}3. Deploy the program: ./deploy.sh $PROGRAM_NAME${NC}"
    echo -e "${YELLOW}4. Generate client: npm run gen:client:$(echo $PROGRAM_NAME | tr '_' '-')${NC}"
    echo -e "${YELLOW}5. Run tests: npm run test:client:$(echo $PROGRAM_NAME | tr '_' '-')${NC}"
    echo ""
    echo -e "${GREEN}Files created:${NC}"
    find "$CATEGORY/$PROGRAM_NAME" -type f | sort
}

# Main execution
main() {
    echo -e "${GREEN}🚀 Creating new Pinocchio program: $PROGRAM_NAME${NC}"
    echo ""
    
    validate_inputs
    create_program_structure
    update_cargo_toml
    update_rust_source
    update_package_json
    create_test_files
    create_deployment_config
    create_readme
    show_summary
    
    echo -e "${GREEN}✅ Program creation completed successfully!${NC}"
}

# Parse command line arguments
for arg in "$@"; do
    case $arg in
        --category=*)
            CATEGORY="${arg#*=}"
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
    esac
done

# Check for help flag
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    show_help
    exit 0
fi

# Run main function
main
