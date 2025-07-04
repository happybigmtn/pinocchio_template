# Pinocchio Template System

A streamlined template system for creating Solana programs with Pinocchio, featuring intelligent aliases and automated workflows.

## 🚀 Quick Start

### Smart Workflow with Bun Aliases

The template system now includes intelligent bun aliases that automatically detect your current context:

```bash
# Create a new program
bun new my-counter --category=basics

# Navigate to your program
cd basics/my-counter

# Make your code changes...

# Generate IDL and TypeScript client
bun gen my-counter --category=basics

# Deploy to devnet
bun dep my-counter --category=basics

# Deploy to testnet
bun dep my-counter --category=basics --network=testnet
```

## 📋 Available Commands

### `bun new` [name] --category=[category] --template=[template]

Creates a new Pinocchio program with all necessary boilerplate:

```bash
# Create basic program using counter template (default)
bun new my-counter

# Create program using account-data template
bun new my-program --template=account-data

# Create token program with account-data template
bun new token-vault --category=tokens --template=account-data

# Create compression program
bun new merkle-tree --category=compression

# Create oracle program
bun new price-feed --category=oracles
```

#### Available Templates

- **`counter` (default)**: Basic counter program template with increment/decrement functionality
- **`account-data`**: Account data management template with create/update operations

#### Template Usage Examples

```bash
# Using counter template (default)
bun new my-counter                              # basics/my-counter
bun new token-counter --category=tokens        # tokens/token-counter

# Using account-data template
bun new user-profile --template=account-data         # basics/user-profile
bun new token-metadata --category=tokens --template=account-data  # tokens/token-metadata
```

### `bun gen` [program-name] --category=[category]

Generates IDL and TypeScript client for the specified program:

```bash
# Generate for basics program
bun gen my-counter --category=basics

# Generate for tokens program
bun gen token-vault --category=tokens

# Auto-detection (shows available programs if none specified)
bun gen
```

### `bun dep` [program-name] --category=[category] --network=[network]

Deploys your program to the specified network:

```bash
# Deploy to devnet (default)
bun dep my-counter --category=basics

# Deploy to testnet
bun dep my-counter --category=basics --network=testnet

# Deploy to mainnet
bun dep my-counter --category=basics --network=mainnet

# Deploy tokens program
bun dep token-vault --category=tokens
```

## 🏗️ What Gets Created

When you create a new program, the template system generates:

### Rust Source Code
- `src/lib.rs` - Main program entry point with declare_id!
- `src/entrypoint.rs` - Program entrypoint
- `src/processor.rs` - Instruction processing logic
- `src/instructions/mod.rs` - Shank-annotated instruction definitions
- `src/state/mod.rs` - Account state definitions with bytemuck traits
- `src/constants.rs` - Program constants

### Configuration Files
- `Cargo.toml` - Package configuration with all necessary dependencies
- `README.md` - Program-specific documentation
- `deploy.sh` - Convenience deployment script

### Test Files
- `tests/[program-name].test.ts` - TypeScript test template using Solana Kite framework
- `tests/[program-name].rs` - Rust test template using Mollusk SVM framework
- `tests/generic*.rs` - Additional Mollusk test utilities and templates

### Build Scripts
- Updates root `package.json` with program-specific scripts:
  - `gen:idl:[program-name]` - Generate IDL using Shank
  - `gen:client:[program-name]` - Generate TypeScript client
  - `test:client:[program-name]` - Run TypeScript tests with Kite
- Cargo.toml includes Mollusk dependencies for Rust testing
- Both test frameworks can be run independently or together

## 🎯 Template Features

### Counter Template

The counter template includes:

#### State Management
- `Counter` struct with proper bytemuck traits for zero-copy deserialization
- Built-in safety methods (increment, decrement with overflow protection)
- Authority-based access control

#### Instructions
- Comprehensive Shank annotations for IDL generation
- Multiple instruction variants (Initialize, Increment, Decrement, SetValue, Reset)
- Proper account documentation

#### Error Handling
- Pinocchio error handling patterns
- Input validation
- Overflow/underflow protection

### Account-Data Template

The account-data template includes:

#### State Management
- `AddressInfo` struct for storing account metadata
- Zero-copy deserialization with bytemuck traits
- Flexible data storage patterns

#### Instructions
- `Create` instruction for initializing new accounts
- Comprehensive account validation
- Shank annotations for IDL generation

#### Features
- Account creation and management patterns
- Proper account size calculations
- Authority and ownership validation
- Extensible for custom data structures

### Testing with Multiple Frameworks

#### TypeScript Testing with Kite
- Uses [Solana Kite](https://solanakite.org) for simplified TypeScript testing
- **Comprehensive Function Demonstrations**: Test templates demonstrate **all 17 core Kite functions**
- **Educational Examples**: Each test serves as both functional testing and learning resource
- **Helius RPC Integration**: Automatically uses high-performance Helius RPC endpoints
- **Complete Coverage**: Wallet management, SOL operations, token lifecycle, transactions & utilities
- Built on top of @solana/kit for modern Solana development
- Enhanced network reliability and faster transaction confirmation

#### Rust Testing with Mollusk
- Uses [Mollusk SVM](https://github.com/buffalojoec/mollusk) for high-performance Rust testing
- **Direct SVM Testing**: Tests run against the Solana Virtual Machine directly
- **Fast Execution**: No RPC calls needed, tests execute locally
- **Comprehensive Utilities**: Account creation, instruction building, assertion helpers
- **Cross-Program Invocation**: Support for testing CPI patterns
- **Performance Monitoring**: Compute unit tracking and resource usage validation
- Built on top of @solana/kit for modern Solana development
- Enhanced network reliability and faster transaction confirmation

#### Kite Functions Demonstrated
Our test templates include working examples of:
- **Wallet Management**: `createWallet()`, `createWallets()`, wallet loading
- **SOL Operations**: `getLamportBalance()`, `airdropIfRequired()`, `transferLamports()`
- **Token Lifecycle**: `createTokenMint()`, `mintTokens()`, `transferTokens()`, balance checking
- **Transactions**: `sendTransactionFromInstructions()`, confirmation, logs, PDA generation
- **Utilities**: Explorer links, account status checking, and more

📚 **See [Kite Functions Guide](docs/KITE_FUNCTIONS.md) for complete TypeScript testing documentation**
📚 **See [Mollusk Test Template Guide](templates/account-data/tests/TEST_TEMPLATE_README.md) for complete Rust testing documentation**

## 📁 Program Categories

Programs are organized into categories:

- `basics/` - Basic Solana programs
- `tokens/` - Token-related programs
- `compression/` - State compression programs
- `oracles/` - Oracle and data feed programs

## 🔄 Development Workflow Examples

### Option 1: Explicit Workflow (Recommended)
```bash
# Create new program with account-data template
bun new token-staking --category=tokens --template=account-data

# Navigate to program directory
cd tokens/token-staking

# Edit your code...
# vim src/lib.rs

# Generate IDL and client
bun gen token-staking --category=tokens

# Deploy to devnet
bun dep token-staking --category=tokens

# Test with TypeScript/Kite framework
bun test

# Or test with Rust/Mollusk framework
cd basics/token-staking && cargo test

# Deploy to mainnet when ready
bun dep token-staking --category=tokens --network=mainnet
```

### Option 2: From Root Directory
```bash
# Create program with specific template
bun new user-rewards --template=account-data

# Generate from root directory
bun gen user-rewards --category=basics

# Deploy from root directory
bun dep user-rewards --category=basics --network=testnet
```

### Option 3: Multiple Programs with Different Templates
```bash
# Create multiple related programs with appropriate templates
bun new user-accounts --template=account-data
bun new voting-system --template=counter
bun new reward-pool --template=account-data

# Work on each one individually
bun gen user-accounts --category=basics && bun dep user-accounts --category=basics
bun gen voting-system --category=basics && bun dep voting-system --category=basics
bun gen reward-pool --category=basics && bun dep reward-pool --category=basics
```

## 🛠️ Legacy Script Access

The original scripts are still available in the `scripts/` directory:

```bash
# Create program (legacy)
./scripts/create-program.sh my-program

# Create with specific template (legacy)
./scripts/create-program.sh my-program --template=account-data --category=basics

# Deploy program (legacy)
./scripts/deploy.sh my-program

# Generate client (legacy)
node scripts/generate-clients.js my-program

# Quick deploy (legacy)
./scripts/quick-deploy.sh my-program
```

## 🌐 RPC Configuration

The template system uses Helius RPC endpoints for improved performance and reliability:

### Default Endpoints
- **Devnet**: `https://devnet.helius-rpc.com/?api-key=YOUR_KEY`
- **Mainnet**: `https://mainnet.helius-rpc.com/?api-key=YOUR_KEY`
- **Testnet**: `https://api.testnet.solana.com` (fallback)
- **Localhost**: `http://localhost:8899`

### Configuration
RPC endpoints are configured in:
- `.env` - Environment variables with your Helius API key
- `scripts/rpc-config.js` - Centralized RPC configuration utility

### Usage
```bash
# List available endpoints
node scripts/rpc-config.js list

# Get specific endpoint
node scripts/rpc-config.js get devnet

# Get WebSocket endpoint
node scripts/rpc-config.js ws devnet
```

## 📁 Template Structure

```
templates/counter/
├── Cargo.toml                 # Package configuration
├── src/
│   ├── lib.rs                # Main entry point
│   ├── entrypoint.rs         # Program entrypoint
│   ├── processor.rs          # Instruction processing
│   ├── constants.rs          # Program constants
│   ├── instructions/
│   │   └── mod.rs           # Instruction definitions
│   └── state/
│       └── mod.rs           # State definitions
└── (generated files)
    ├── tests/               # Test files
    ├── deploy.sh           # Deployment script
    └── README.md           # Program documentation
```

## 🔧 Creating Custom Templates

To create new templates:

1. **Create Template Directory**: Add your template in `templates/` or use an existing program as a template
2. **Add Rust Source Files**: Include all necessary `.rs` files with proper structure
3. **Use Placeholder Names**: Use consistent naming (like "counter" or "account_data") that will be replaced
4. **Update Script**: Add your template to the `create-program.sh` script validation section

### Template Structure Requirements

```
your-template/
├── Cargo.toml                 # Package configuration
├── src/
│   ├── lib.rs                # Main entry with declare_id!
│   ├── entrypoint.rs         # Program entrypoint
│   ├── processor.rs          # Instruction processing
│   ├── constants.rs          # Program constants (optional)
│   ├── instructions/
│   │   └── mod.rs           # Instruction definitions
│   └── state/
│       └── mod.rs           # State definitions
└── tests/
    └── *.rs                 # Test files
```

### Adding New Templates

To add a new template called `my-template`:

1. Create the template structure in `templates/my-template/` or `basics/my-template/`
2. Update `scripts/create-program.sh` validation section:

```bash
case $TEMPLATE_NAME in
    counter)
        TEMPLATE_DIR="templates/counter"
        ;;
    account-data)
        TEMPLATE_DIR="basics/account-data"
        ;;
    my-template)  # Add this
        TEMPLATE_DIR="templates/my-template"
        ;;
    *)
        echo "Unknown template..."
        ;;
esac
```

3. Update the help text and documentation

## 🚨 Troubleshooting

### Program Already Exists
If you see "Program already exists", either:
- Choose a different name
- Remove the existing program directory
- Use a different category

### Deployment Fails
- Check your Solana CLI configuration: `solana config get`
- Ensure you have sufficient SOL for deployment
- Verify network connectivity
- Check Helius API key in `.env` file

### Client Generation Fails
- Ensure the program builds successfully first
- Check that Shank annotations are correct
- Verify IDL generation works: `bun generate` or manually check IDL output

### Auto-Detection Not Working
- Ensure you're in a valid program directory (`category/program-name`)
- Use explicit program names if auto-detection fails
- Check directory structure matches expected pattern

## 🎯 Help Commands

Each command supports `--help` for detailed usage:

```bash
bun new --help             # Create command help with template options
bun gen --help             # Generate command help  
bun dep --help             # Deploy command help

# See available templates
./scripts/create-program.sh --help
```

## 📚 Integration with Existing Workflow

This template system integrates with your existing scripts:

- `scripts/deploy.sh` - Handles building and deploying any program with Helius RPC
- `scripts/update-program-ids.sh` - Updates program IDs after deployment
- `scripts/generate-clients.js` - Generates TypeScript clients
- `scripts/rpc-config.js` - Centralized RPC endpoint management

The new bun aliases extend these tools to work seamlessly with intelligent context detection and simplified commands.
