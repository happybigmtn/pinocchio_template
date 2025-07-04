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
            TEMPLATE_DIR="basics/counter"
            ;;
        account-data)
            TEMPLATE_DIR="basics/account-data"
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
    
    # Determine the original template name to replace in Cargo.toml
    local template_cargo_name
    case $TEMPLATE_NAME in
        counter)
            template_cargo_name="counter"
            ;;
        account-data)
            template_cargo_name="account-data"
            ;;
        *)
            template_cargo_name="counter"  # fallback
            ;;
    esac
    
    # Update program Cargo.toml
    if [ -f "$target_dir/Cargo.toml" ]; then
        sed -i "s/name = \"$template_cargo_name\"/name = \"$PROGRAM_NAME\"/g" "$target_dir/Cargo.toml"
        sed -i "s/name = \"${template_cargo_name}_\"/name = \"${program_name_snake}_\"/g" "$target_dir/Cargo.toml"
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
    
    # Delete existing test files from template
    if [ -d "$target_dir/tests" ]; then
        echo -e "${YELLOW}Removing existing test files...${NC}"
        rm -rf "$target_dir/tests"/*
    fi
    
    # Create tests directory if it doesn't exist
    mkdir -p "$target_dir/tests"
    
    # Create TypeScript test file with comprehensive Kite function demonstrations
    # Using functions from Solana Kite: https://github.com/helius-labs/kite-og
    cat > "$target_dir/tests/$program_name_dash.test.ts" << EOF
import { describe, test, beforeAll } from 'bun:test';
import { connect } from 'solana-kite';
import { address, lamports } from '@solana/kit';
import { getTransferSolInstruction } from '@solana-program/system';

// Import generated client (will be available after running gen:client)
// import { ${program_name_snake}Program } from '../clients/${program_name_dash}';

describe('${PROGRAM_NAME} - Comprehensive Kite Demo', () => {
  let kite: Awaited<ReturnType<typeof connect>>;
  const programId = address('11111111111111111111111111111111'); // Will be updated after deployment

  beforeAll(async () => {
    // Use standard Solana devnet RPC for testing (Helius may have restrictions)
    // For production, use Helius RPC from the config
    const rpcEndpoint = 'https://api.devnet.solana.com';
    const wsEndpoint = 'wss://api.devnet.solana.com';
    kite = await connect(rpcEndpoint, wsEndpoint);
  });

  test('should demonstrate all Kite wallet functions', async () => {
    console.log('\n🔑 === WALLET MANAGEMENT FUNCTIONS ===');
    
    try {
      // 1. createWallet - Create a new wallet
      console.log('\n1️⃣  Creating wallets with different options...');
      
      const basicWallet = await kite.createWallet();
      console.log('Basic wallet created:', basicWallet.address);
      
      const customWallet = await kite.createWallet({ 
        airdropAmount: lamports(2_000_000_000n), // 2 SOL
        prefix: 'COOL',
        suffix: 'TEST'
      });
      console.log('Custom wallet created with prefix/suffix:', customWallet.address);
      
      // 2. createWallets - Create multiple wallets at once
      console.log('\n2️⃣  Creating multiple wallets...');
      const multipleWallets = await kite.createWallets(3, {
        airdropAmount: lamports(500_000_000n) // 0.5 SOL each
      });
      console.log('Created', multipleWallets.length, 'wallets:', multipleWallets.map(w => w.address));
      
      console.log('✅ Wallet creation functions working!');
    } catch (error) {
      console.error('❌ Wallet functions error:', error);
    }
  }, { timeout: 120000 });

  test('should demonstrate SOL balance and transfer functions', async () => {
    console.log('\n💰 === SOL MANAGEMENT FUNCTIONS ===');
    
    try {
      // Create test wallets
      const sender = await kite.createWallet({ 
        airdropAmount: lamports(2_000_000_000n) // 2 SOL
      });
      const receiver = await kite.createWallet();
      
      // 3. getLamportBalance - Get SOL balance
      console.log('\n3️⃣  Checking balances...');
      const senderBalance = await kite.getLamportBalance(sender.address);
      const receiverBalance = await kite.getLamportBalance(receiver.address);
      console.log('Sender balance:', Number(senderBalance) / 1_000_000_000, 'SOL');
      console.log('Receiver balance:', Number(receiverBalance) / 1_000_000_000, 'SOL');
      
      // 4. airdropIfRequired - Conditional airdrop
      console.log('\n4️⃣  Testing conditional airdrop...');
      const minimumBalance = lamports(1_000_000_000n); // 1 SOL
      const airdropAmount = lamports(1_500_000_000n); // 1.5 SOL
      
      const airdropSig = await kite.airdropIfRequired(
        receiver.address,
        airdropAmount,
        minimumBalance
      );
      
      if (airdropSig) {
        console.log('Airdrop completed, signature:', airdropSig);
      } else {
        console.log('No airdrop needed, sufficient balance');
      }
      
      // 5. transferLamports - Transfer SOL between wallets
      console.log('\n5️⃣  Transferring SOL...');
      const transferAmount = lamports(250_000_000n); // 0.25 SOL
      const transferSig = await kite.transferLamports({
        source: sender,
        destination: receiver.address,
        amount: transferAmount,
        skipPreflight: false,
        maximumClientSideRetries: 3
      });
      
      console.log('SOL transfer completed, signature:', transferSig);
      
      // Check balances after transfer
      const newSenderBalance = await kite.getLamportBalance(sender.address);
      const newReceiverBalance = await kite.getLamportBalance(receiver.address);
      console.log('New sender balance:', Number(newSenderBalance) / 1_000_000_000, 'SOL');
      console.log('New receiver balance:', Number(newReceiverBalance) / 1_000_000_000, 'SOL');
      
      console.log('✅ SOL management functions working!');
    } catch (error) {
      console.error('❌ SOL functions error:', error);
    }
  }, { timeout: 120000 });

  test('should demonstrate token functions', async () => {
    console.log('\n🪙 === TOKEN MANAGEMENT FUNCTIONS ===');
    
    try {
      // Create a wallet to be the mint authority
      const mintAuthority = await kite.createWallet({ 
        airdropAmount: lamports(2_000_000_000n) // 2 SOL
      });
      
      // 6. createTokenMint - Create a new token
      console.log('\n6️⃣  Creating a new token mint...');
      const mintAddress = await kite.createTokenMint({
        mintAuthority,
        decimals: 9,
        name: 'Test Token',
        symbol: 'TEST',
        uri: 'https://example.com/token.json',
        additionalMetadata: {
          description: 'A test token created with Kite',
          category: 'utility'
        }
      });
      console.log('Token mint created:', mintAddress);
      
      // 7. getMint - Get token mint information
      console.log('\n7️⃣  Getting mint information...');
      const mintInfo = await kite.getMint(mintAddress);
      console.log('Mint info - decimals:', mintInfo.decimals, 'supply:', mintInfo.supply);
      
      // 8. getTokenAccountAddress - Get token account address
      console.log('\n8️⃣  Getting token account addresses...');
      const authorityTokenAccount = await kite.getTokenAccountAddress(
        mintAuthority.address,
        mintAddress
      );
      console.log('Mint authority token account:', authorityTokenAccount);
      
      // Create a recipient wallet
      const recipient = await kite.createWallet({ 
        airdropAmount: lamports(1_000_000_000n) // 1 SOL
      });
      
      const recipientTokenAccount = await kite.getTokenAccountAddress(
        recipient.address,
        mintAddress
      );
      console.log('Recipient token account:', recipientTokenAccount);
      
      // 9. mintTokens - Mint tokens to an account
      console.log('\n9️⃣  Minting tokens...');
      const mintAmount = 1000n * 10n ** 9n; // 1000 tokens with 9 decimals
      const mintSig = await kite.mintTokens(
        mintAddress,
        mintAuthority,
        mintAmount,
        mintAuthority.address
      );
      console.log('Tokens minted, signature:', mintSig);
      
      // 10. getTokenAccountBalance - Get token account balance
      console.log('\n🔟 Getting token balances...');
      const authorityBalance = await kite.getTokenAccountBalance(authorityTokenAccount);
      console.log('Authority token balance:', Number(authorityBalance.amount) / 10**9, 'tokens');
      
      // 11. transferTokens - Transfer tokens between accounts
      console.log('\n1️⃣1️⃣ Transferring tokens...');
      const transferAmount = 100n * 10n ** 9n; // 100 tokens
      const tokenTransferSig = await kite.transferTokens({
        sender: mintAuthority,
        destination: recipient.address,
        mintAddress,
        amount: transferAmount,
        maximumClientSideRetries: 3
      });
      console.log('Tokens transferred, signature:', tokenTransferSig);
      
      // Check balances after transfer
      const newAuthorityBalance = await kite.getTokenAccountBalance(authorityTokenAccount);
      const recipientBalance = await kite.getTokenAccountBalance(recipientTokenAccount);
      console.log('New authority balance:', Number(newAuthorityBalance.amount) / 10**9, 'tokens');
      console.log('Recipient balance:', Number(recipientBalance.amount) / 10**9, 'tokens');
      
      // 12. checkTokenAccountIsClosed - Check if token account is closed
      console.log('\n1️⃣2️⃣ Checking if token accounts are closed...');
      const isAuthorityClosed = await kite.checkTokenAccountIsClosed(authorityTokenAccount);
      const isRecipientClosed = await kite.checkTokenAccountIsClosed(recipientTokenAccount);
      console.log('Authority account closed:', isAuthorityClosed);
      console.log('Recipient account closed:', isRecipientClosed);
      
      console.log('✅ Token management functions working!');
    } catch (error) {
      console.error('❌ Token functions error:', error);
    }
  }, { timeout: 180000 });

  test('should demonstrate transaction and utility functions', async () => {
    console.log('\n⚙️ === TRANSACTION & UTILITY FUNCTIONS ===');
    
    try {
      const wallet = await kite.createWallet({ 
        airdropAmount: lamports(2_000_000_000n) // 2 SOL
      });
      const recipient1 = await kite.createWallet();
      const recipient2 = await kite.createWallet();
      
      // 13. sendTransactionFromInstructions - Send transaction with multiple instructions
      console.log('\n1️⃣3️⃣ Sending transaction with multiple instructions...');
      
      const instruction1 = getTransferSolInstruction({
        amount: lamports(50_000_000n), // 0.05 SOL
        destination: recipient1.address,
        source: wallet
      });
      
      const instruction2 = getTransferSolInstruction({
        amount: lamports(75_000_000n), // 0.075 SOL
        destination: recipient2.address,
        source: wallet
      });
      
      const multiInstructionSig = await kite.sendTransactionFromInstructions({
        feePayer: wallet,
        instructions: [instruction1, instruction2],
        commitment: 'confirmed',
        skipPreflight: false,
        maximumClientSideRetries: 3
      });
      
      console.log('Multi-instruction transaction completed:', multiInstructionSig);
      
      // 14. getRecentSignatureConfirmation - Check transaction confirmation
      console.log('\n1️⃣4️⃣ Checking transaction confirmation...');
      const isConfirmed = await kite.getRecentSignatureConfirmation(multiInstructionSig);
      console.log('Transaction confirmed:', isConfirmed);
      
      // 15. getLogs - Get transaction logs
      console.log('\n1️⃣5️⃣ Getting transaction logs...');
      const logs = await kite.getLogs(multiInstructionSig);
      console.log('Transaction logs:', logs.slice(0, 3), '... (showing first 3)');
      
      // 16. getPDAAndBump - Get Program Derived Address
      console.log('\n1️⃣6️⃣ Getting PDA and bump seed...');
      const seeds = [Buffer.from('test'), wallet.address.toBytes()];
      const [pda, bump] = await kite.getPDAAndBump(seeds, programId);
      console.log('PDA:', pda);
      console.log('Bump seed:', bump);
      
      // 17. getExplorerLink - Get explorer links for different entities
      console.log('\n1️⃣7️⃣ Getting explorer links...');
      const addressLink = kite.getExplorerLink('address', wallet.address);
      const transactionLink = kite.getExplorerLink('transaction', multiInstructionSig);
      const blockLink = kite.getExplorerLink('block', '12345');
      
      console.log('Explorer links:');
      console.log('  Address:', addressLink);
      console.log('  Transaction:', transactionLink);
      console.log('  Block:', blockLink);
      
      console.log('✅ Transaction and utility functions working!');
    } catch (error) {
      console.error('❌ Transaction/utility functions error:', error);
    }
  }, { timeout: 120000 });

  test('should demonstrate program-specific functionality', async () => {
    console.log('\n🔧 === PROGRAM-SPECIFIC TESTS ===');
    console.log('TODO: Add tests specific to ${PROGRAM_NAME} program functionality');
    console.log('Program ID:', programId);
    
    try {
      // TODO: Add program-specific tests here
      // Example:
      // const wallet = await kite.createWallet({ airdropAmount: lamports(1_000_000_000n) });
      // const instruction = create${PROGRAM_NAME}Instruction({ ... });
      // const signature = await kite.sendTransactionFromInstructions({
      //   feePayer: wallet,
      //   instructions: [instruction]
      // });
      
      console.log('✅ Program-specific tests ready for implementation!');
    } catch (error) {
      console.error('❌ Program-specific test error:', error);
    }
  }, { timeout: 60000 });
});
EOF
    
    echo -e "${GREEN}✓ Test files created with proper Kite functions${NC}"
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
