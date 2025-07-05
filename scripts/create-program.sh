#!/bin/bash

# Script to create new Pinocchio programs from template
# Usage: ./create-program.sh [program_name] [--category=basics|tokens|compression|oracles] [--template=template_name]

set -e

# Default values
PROGRAM_NAME="$1"
CATEGORY="basics"
TEMPLATE_NAME="counter"
TEMPLATE_DIR=""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Help function
show_help() {
    echo "Usage: $0 [program_name] [--category=basics|tokens|compression|oracles] [--template=template_name]"
    echo ""
    echo "Creates a new Pinocchio program from template"
    echo ""
    echo "Arguments:"
    echo "  program_name         Name of the new program (required)"
    echo "  --category=CATEGORY  Category for the program (default: basics)"
    echo "  --template=TEMPLATE  Template to use (default: counter)"
    echo ""
    echo "Examples:"
    echo "  $0 my_program                              # Create basics/my_program from counter template"
    echo "  $0 token_mint --category=tokens            # Create tokens/token_mint from counter template"
    echo "  $0 my_counter --template=account-data      # Create basics/my_counter from account-data template"
    echo "  $0 my_program --category=tokens --template=account-data  # Create tokens/my_program from account-data template"
    echo ""
    echo "Available templates:"
    echo "  - counter (default): Basic counter program template"
    echo "  - account-data: Account data management template"
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

    # Set template directory based on template name
    case $TEMPLATE_NAME in
        counter)
            TEMPLATE_DIR="templates/account-data"  # Use account-data as base for counter template
            ;;
        account-data)
            TEMPLATE_DIR="templates/account-data"
            ;;
        *)
            echo -e "${RED}Error: Unknown template '$TEMPLATE_NAME'. Available templates: counter, account-data${NC}"
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
    
    # Update program Cargo.toml - replace any template name patterns with the new program name
    if [ -f "$target_dir/Cargo.toml" ]; then
        # Replace common template name patterns
        sed -i "s/name = \"account-data-template\"/name = \"$PROGRAM_NAME\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"counter-template\"/name = \"$PROGRAM_NAME\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"account-data\"/name = \"$PROGRAM_NAME\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"counter\"/name = \"$PROGRAM_NAME\"/g" "$target_dir/Cargo.toml"
        
        # Replace with snake_case version if needed
        sed -i "s/name = \"account_data_template\"/name = \"$program_name_snake\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"counter_template\"/name = \"$program_name_snake\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"account_data\"/name = \"$program_name_snake\"/g" "$target_dir/Cargo.toml"
        
        echo -e "${GREEN}✓ Updated Cargo.toml package name to $PROGRAM_NAME${NC}"
    fi
    
    # Update workspace Cargo.toml
    if [ -f "Cargo.toml" ]; then
        # Check if we need to add explicit member or if wildcard already covers it
        local category_wildcard="\"$CATEGORY/*\""
        
        # If category wildcard doesn't exist, add it
        if ! grep -q "$category_wildcard" Cargo.toml; then
            echo -e "${YELLOW}Adding $CATEGORY category to workspace members...${NC}"
            # Add the category wildcard to members array
            sed -i "/members = \[/,/\]/ s/\]/  \"$CATEGORY\/*\",\n\]/" Cargo.toml
        else
            echo -e "${GREEN}✓ $CATEGORY category already in workspace members${NC}"
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
    
    # Determine the original template name to replace
    local template_name_snake
    case $TEMPLATE_NAME in
        counter)
            template_name_snake="counter"
            ;;
        account-data)
            template_name_snake="account_data"
            ;;
        *)
            template_name_snake="counter"  # fallback
            ;;
    esac
    
    local template_name_upper=$(echo "$template_name_snake" | tr '[:lower:]' '[:upper:]')
    
    # Update lib.rs
    if [ -f "$target_dir/src/lib.rs" ]; then
        sed -i "s/$template_name_snake/$program_name_snake/g" "$target_dir/src/lib.rs"
        sed -i "s/$template_name_upper/$program_name_upper/g" "$target_dir/src/lib.rs"
        sed -i "s/pinocchio_pubkey::declare_id!(\"[^\"]*\")/pinocchio_pubkey::declare_id!(\"$new_program_id\")/g" "$target_dir/src/lib.rs"
    fi
    
    # Update other Rust files
    find "$target_dir/src" -name "*.rs" -exec sed -i "s/$template_name_snake/$program_name_snake/g" {} \;
    find "$target_dir/src" -name "*.rs" -exec sed -i "s/$template_name_upper/$program_name_upper/g" {} \;
    
    # Also update test files
    find "$target_dir/tests" -name "*.rs" -exec sed -i "s/$template_name_snake/$program_name_snake/g" {} \; 2>/dev/null || true
    find "$target_dir/tests" -name "*.rs" -exec sed -i "s/$template_name_upper/$program_name_upper/g" {} \; 2>/dev/null || true
    
    
    echo -e "${GREEN}✓ Rust source files updated${NC}"
}

# Update package.json scripts
update_package_json() {
    echo -e "${BLUE}Updating package.json scripts...${NC}"
    
    # Call the package script updater
    chmod +x "./scripts/update-package-scripts.sh"
    "./scripts/update-package-scripts.sh" add "$PROGRAM_NAME" "$CATEGORY"
    
    echo -e "${GREEN}✓ package.json scripts updated${NC}"
}

# Create test files
create_test_files() {
    echo -e "${BLUE}Creating test files...${NC}"
    
    local target_dir="$CATEGORY/$PROGRAM_NAME"
    local program_name_dash=$(echo "$PROGRAM_NAME" | tr '_' '-')
    local program_name_snake=$(echo "$PROGRAM_NAME" | tr '-' '_')
    local program_name_title=$(echo "$PROGRAM_NAME" | tr '_-' ' ' | sed 's/\b\w/\U&/g')
    
    # Create tests directory if it doesn't exist
    mkdir -p "$target_dir/tests"
    
    local template_test_dir="templates/account-data/tests"
    
    # Copy utility files and documentation if they exist
    if [ -d "$template_test_dir" ]; then
        echo -e "${YELLOW}Setting up test documentation and utilities...${NC}"
        
        # Copy generic test utilities (keep these functional)
        if [ -f "$template_test_dir/genericmollusk.rs" ]; then
            cp "$template_test_dir/genericmollusk.rs" "$target_dir/tests/"
            echo -e "${GREEN}✓ Copied Mollusk test utilities${NC}"
        fi
        
        if [ -f "$template_test_dir/generickite.ts" ]; then
            cp "$template_test_dir/generickite.ts" "$target_dir/tests/"
            echo -e "${GREEN}✓ Copied Kite test utilities${NC}"
        fi
        
        # Copy comprehensive test documentation
        if [ -f "$template_test_dir/TEST_TEMPLATE_README.md" ]; then
            cp "$template_test_dir/TEST_TEMPLATE_README.md" "$target_dir/tests/"
            echo -e "${GREEN}✓ Copied test documentation${NC}"
        fi
    fi
    
    # Create Rust test guide as markdown
    echo -e "${YELLOW}Creating Rust test guide (markdown)...${NC}"
    cat > "$target_dir/tests/rust-test-guide.md" << EOF
# Rust Testing Guide for ${program_name_title}

This guide shows how to write Mollusk SVM tests for your ${program_name_title} program.

## Example Test Structure

\`\`\`rust
#[cfg(test)]
mod tests {
    use ${program_name_snake}::{
        // Import your program's types and constants
        state::{YourState, YourInstructionData},
        ID,
    };
    use mollusk_svm::{
        result::{Check, ProgramResult},
        Mollusk,
    };
    use pinocchio_helper::create_padded_array;
    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
    };

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    #[test]
    fn test_your_instruction() {
        // 1. Initialize Mollusk with your program
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/${program_name_snake}");

        // 2. Create test accounts
        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let owner = Pubkey::new_from_array([0x02; 32]);
        let owner_account = AccountSharedData::new(LAMPORTS_PER_SOL, 0, &system_program);

        let data_account_pubkey = Pubkey::new_unique();
        let data_account = AccountSharedData::new(0, 0, &system_program);

        // 3. Create instruction data
        let ix_data = YourInstructionData {
            // Fill in your instruction data fields
        };

        let ix_data_bytes = bytemuck::bytes_of(&ix_data);
        let data = [vec![0], ix_data_bytes.to_vec()].concat(); // 0 = instruction index

        // 4. Build instruction
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &data,
            vec![
                AccountMeta::new(owner, true),
                AccountMeta::new(data_account_pubkey, true),
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        // 5. Execute and validate
        let result = mollusk
            .process_and_validate_instruction(
                &instruction,
                &[
                    (owner, owner_account.into()),
                    (data_account_pubkey, data_account.into()),
                    (system_program, system_account),
                ],
                &[
                    Check::success(),
                    Check::account(&data_account_pubkey)
                        .data(ix_data_bytes)
                        .build(),
                ],
            );

        // 6. Additional assertions
        let updated_data = result.get_account(&data_account_pubkey).unwrap();
        let parsed_data = bytemuck::from_bytes::<YourState>(&updated_data.data);
        
        // Add your specific assertions here
        assert!(result.program_result == ProgramResult::Success);
    }

    #[test]
    fn test_error_conditions() {
        // Test error cases
        let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/${program_name_snake}");
        
        // Create test scenario that should fail
        // Use mollusk.process_instruction() and assert on expected errors
    }
}
\`\`\`

## Key Points

1. **Import your program's types**: Update the \`use\` statements to match your program structure
2. **Update program binary path**: Ensure the path matches your compiled program
3. **Create realistic test data**: Use actual data structures from your program
4. **Test both success and failure cases**: Include error condition testing
5. **Use appropriate account configurations**: Match your program's account requirements

## Running Tests

\`\`\`bash
# Run all tests
cargo test

# Run specific test
cargo test test_your_instruction

# Run with output
cargo test -- --nocapture
\`\`\`

## Additional Resources

- See \`genericmollusk.rs\` for utility functions
- Check \`TEST_TEMPLATE_README.md\` for comprehensive testing guide
- Mollusk documentation: https://github.com/buffalojoec/mollusk
EOF

    # Create TypeScript test guide as markdown
    echo -e "${YELLOW}Creating TypeScript test guide (markdown)...${NC}"
    cat > "$target_dir/tests/typescript-test-guide.md" << EOF
# TypeScript Testing Guide for ${program_name_title}

This guide shows how to write Solana Kite tests for your ${program_name_title} program.

## Example Test Structure

\`\`\`typescript
import { expect, test } from 'bun:test';
import { connect } from 'solana-kite';
import { generateKeyPairSigner } from '@solana/kit';
// Import your generated client (after running gen:client)
// import { fetchYourData, getYourInstruction } from '../../../clients/${program_name_dash}';
import { getKiteConnection } from '../../../scripts/rpc-config.js';
import dotenv from 'dotenv';
import { join } from 'path';

// Load environment variables
const envPath = join(__dirname, '../../../.env');
dotenv.config({ path: envPath });

test('${program_name_dash}:infrastructure', async () => {
  console.log('🧪 Testing ${program_name_dash} program infrastructure');
  
  // Connect to devnet for testing
  const rpcEndpoint = 'https://api.devnet.solana.com';
  const wsEndpoint = 'wss://api.devnet.solana.com';
  const kite = await connect(rpcEndpoint, wsEndpoint);
  console.log('✅ Connected to devnet successfully');
  
  // Test basic RPC connectivity
  const version = await kite.rpc.getVersion().send();
  console.log('✅ RPC version:', version['solana-core']);
  
  // Check program binary exists
  const fs = require('fs');
  const programBinary = '../../target/deploy/${program_name_snake}.so';
  if (fs.existsSync(programBinary)) {
    console.log('✅ Program binary exists and is ready for deployment');
  } else {
    console.log('⚠️  Program binary not found - run cargo build-sbf first');
  }
  
  expect(version['solana-core']).toBeTruthy();
  console.log('✅ Infrastructure test passed!');
}, { timeout: 30000 });

test('${program_name_dash}:functionality', async () => {
  console.log('🔧 Testing ${program_name_dash} program functionality');
  
  const kite = await connect('https://api.devnet.solana.com', 'wss://api.devnet.solana.com');
  
  // Create test wallet
  const wallet = await kite.createWallet({ 
    airdropAmount: lamports(1_000_000_000n) // 1 SOL
  });
  
  // TODO: Add your program-specific tests here
  // Example:
  // const instruction = getYourInstruction({
  //   // your instruction parameters
  // });
  // 
  // const signature = await kite.sendTransactionFromInstructions({
  //   feePayer: wallet,
  //   instructions: [instruction],
  //   commitment: 'confirmed'
  // });
  // 
  // console.log('Transaction completed:', signature);
  
  console.log('✅ Add your functionality tests here!');
}, { timeout: 60000 });
\`\`\`

## Comprehensive Kite Functions Demo

\`\`\`typescript
import { describe, test, beforeAll } from 'bun:test';
import { connect } from 'solana-kite';
import { address, lamports } from '@solana/kit';
import { getTransferSolInstruction } from '@solana-program/system';

describe('${program_name_title} - Kite Functions Demo', () => {
  let kite: Awaited<ReturnType<typeof connect>>;
  const programId = address('11111111111111111111111111111111'); // Update after deployment

  beforeAll(async () => {
    const rpcEndpoint = 'https://api.devnet.solana.com';
    const wsEndpoint = 'wss://api.devnet.solana.com';
    kite = await connect(rpcEndpoint, wsEndpoint);
  });

  test('wallet management', async () => {
    // 1. createWallet - Create wallets with options
    const basicWallet = await kite.createWallet();
    const customWallet = await kite.createWallet({ 
      airdropAmount: lamports(2_000_000_000n), // 2 SOL
      prefix: 'TEST'
    });
    
    // 2. createWallets - Create multiple wallets
    const multipleWallets = await kite.createWallets(3, {
      airdropAmount: lamports(500_000_000n) // 0.5 SOL each
    });
    
    console.log('Created wallets:', multipleWallets.map(w => w.address));
  });

  test('SOL management', async () => {
    const sender = await kite.createWallet({ airdropAmount: lamports(2_000_000_000n) });
    const receiver = await kite.createWallet();
    
    // 3. getLamportBalance - Check balances
    const senderBalance = await kite.getLamportBalance(sender.address);
    console.log('Sender balance:', Number(senderBalance) / 1_000_000_000, 'SOL');
    
    // 4. airdropIfRequired - Conditional airdrop
    await kite.airdropIfRequired(
      receiver.address,
      lamports(1_500_000_000n),
      lamports(1_000_000_000n)
    );
    
    // 5. transferLamports - Transfer SOL
    const transferSig = await kite.transferLamports({
      source: sender,
      destination: receiver.address,
      amount: lamports(250_000_000n)
    });
    console.log('Transfer completed:', transferSig);
  });

  test('token management', async () => {
    const mintAuthority = await kite.createWallet({ airdropAmount: lamports(2_000_000_000n) });
    
    // 6. createTokenMint - Create new token
    const mintAddress = await kite.createTokenMint({
      mintAuthority,
      decimals: 9,
      name: 'Test Token',
      symbol: 'TEST'
    });
    
    // 7. getMint - Get mint info
    const mintInfo = await kite.getMint(mintAddress);
    console.log('Mint decimals:', mintInfo.decimals);
    
    // 8. getTokenAccountAddress - Get token account
    const tokenAccount = await kite.getTokenAccountAddress(
      mintAuthority.address,
      mintAddress
    );
    
    // 9. mintTokens - Mint tokens
    const mintAmount = 1000n * 10n ** 9n; // 1000 tokens
    await kite.mintTokens(mintAddress, mintAuthority, mintAmount, mintAuthority.address);
    
    // 10. getTokenAccountBalance - Check balance
    const balance = await kite.getTokenAccountBalance(tokenAccount);
    console.log('Token balance:', Number(balance.amount) / 10**9);
    
    // 11. transferTokens - Transfer tokens
    const recipient = await kite.createWallet({ airdropAmount: lamports(1_000_000_000n) });
    await kite.transferTokens({
      sender: mintAuthority,
      destination: recipient.address,
      mintAddress,
      amount: 100n * 10n ** 9n
    });
  });

  test('transaction utilities', async () => {
    const wallet = await kite.createWallet({ airdropAmount: lamports(2_000_000_000n) });
    const recipient = await kite.createWallet();
    
    // 12. sendTransactionFromInstructions - Multi-instruction transaction
    const instruction = getTransferSolInstruction({
      amount: lamports(50_000_000n),
      destination: recipient.address,
      source: wallet
    });
    
    const signature = await kite.sendTransactionFromInstructions({
      feePayer: wallet,
      instructions: [instruction],
      commitment: 'confirmed'
    });
    
    // 13. getRecentSignatureConfirmation - Check confirmation
    const isConfirmed = await kite.getRecentSignatureConfirmation(signature);
    console.log('Transaction confirmed:', isConfirmed);
    
    // 14. getLogs - Get transaction logs
    const logs = await kite.getLogs(signature);
    console.log('Logs count:', logs.length);
    
    // 15. getPDAAndBump - Get Program Derived Address
    const seeds = [Buffer.from('test'), wallet.address.toBytes()];
    const [pda, bump] = await kite.getPDAAndBump(seeds, programId);
    console.log('PDA:', pda, 'Bump:', bump);
    
    // 16. getExplorerLink - Get explorer links
    const addressLink = kite.getExplorerLink('address', wallet.address);
    const txLink = kite.getExplorerLink('transaction', signature);
    console.log('Explorer links generated');
  });
});
\`\`\`

## Key Points

1. **Environment Setup**: Configure .env with HELIUS_API_KEY for production
2. **Client Generation**: Run \`bun gen\` to generate TypeScript client before testing
3. **Network Selection**: Use devnet for testing, production RPC for deployment
4. **Timeout Configuration**: Network tests need longer timeouts (30-60 seconds)
5. **Error Handling**: Always handle network errors and timeouts gracefully

## Running Tests

\`\`\`bash
# Generate client first
bun gen

# Run TypeScript tests
bun test:client:${program_name_dash}

# Run all tests
bun test

# Run with verbose output
bun test --verbose
\`\`\`

## Additional Resources

- See \`generickite.ts\` for utility functions
- Check \`TEST_TEMPLATE_README.md\` for comprehensive testing guide
- Solana Kite documentation: https://github.com/helius-labs/kite-og
EOF
    
    # Create skeleton test files for users to implement
    echo -e "${YELLOW}Creating skeleton test files...${NC}"
    
    # Create skeleton Rust test file
    cat > "$target_dir/tests/${program_name_snake}.rs" << EOF
// Skeleton test file for ${program_name_title}
// See rust-test-guide.md for examples and implementation guide

#[cfg(test)]
mod tests {
    use ${program_name_snake}::{
        // TODO: Import your program's types and constants
        // state::{YourState, YourInstructionData},
        ID,
    };
    use mollusk_svm::{
        result::{Check, ProgramResult},
        Mollusk,
    };
    use solana_sdk::{
        account::AccountSharedData,
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
    };

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    #[test]
    fn test_placeholder() {
        // TODO: Implement your tests here
        // See rust-test-guide.md for examples
        
        // Basic program initialization test
        let _mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/${program_name_snake}");
        
        // This test passes by default - replace with actual tests
        assert!(true);
    }
    
    // TODO: Add more test functions
    // #[test]
    // fn test_your_instruction() {
    //     // Implement your instruction tests
    // }
    //
    // #[test] 
    // fn test_error_conditions() {
    //     // Test error cases
    // }
}
EOF

    # Create skeleton TypeScript test file
    cat > "$target_dir/tests/${program_name_dash}.test.ts" << EOF
// Skeleton test file for ${program_name_title}
// See typescript-test-guide.md for examples and implementation guide

import { expect, test } from 'bun:test';
import { connect } from 'solana-kite';
// TODO: Import your generated client after running 'bun gen'
// import { fetchYourData, getYourInstruction } from '../../../clients/${program_name_dash}';

test('${program_name_dash}:placeholder', async () => {
  console.log('🧪 ${program_name_title} - Placeholder test');
  
  // TODO: Implement your tests here
  // See typescript-test-guide.md for examples
  
  // Basic connectivity test
  const kite = await connect('https://api.devnet.solana.com', 'wss://api.devnet.solana.com');
  const version = await kite.rpc.getVersion().send();
  
  expect(version['solana-core']).toBeTruthy();
  console.log('✅ Basic connectivity test passed!');
  console.log('💡 See typescript-test-guide.md to implement your program tests');
}, { timeout: 30000 });

// TODO: Add more test functions
// test('${program_name_dash}:your-functionality', async () => {
//   // Implement your program-specific tests
// });
EOF

    echo -e "${GREEN}✓ Created skeleton test files${NC}"
    echo -e "${GREEN}✓ Created markdown test guides${NC}"
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
exec "./scripts/deploy.sh" "$PROGRAM_NAME" "$@"
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
    echo -e "${GREEN}📋 Using template: $TEMPLATE_NAME${NC}"
    echo -e "${GREEN}📁 Target category: $CATEGORY${NC}"
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
        --template=*)
            TEMPLATE_NAME="${arg#*=}"
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
