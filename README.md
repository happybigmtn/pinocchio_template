# Pinocchio Template System

A streamlined template system for creating Solana programs with Pinocchio, featuring intelligent aliases and automated workflows.

## 🚀 Quick Start

### Smart Workflow with Bun Aliases

The template system now includes intelligent bun aliases that automatically detect your current context:

```bash
# Create a new program
bun create my-counter --category=basics

# Navigate to your program
cd basics/my-counter

# Make your code changes...

# Generate IDL and TypeScript client (auto-detects program)
bun generate

# Deploy to devnet (auto-detects program)
bun run deploy

# Deploy to testnet
bun run deploy --network=testnet
```

## 📋 Available Commands

### `bun create [name] --category=[category]`

Creates a new Pinocchio program with all necessary boilerplate:

```bash
# Create basic program (default category: basics)
bun create my-counter

# Create token program
bun create token-vault --category=tokens

# Create compression program
bun create merkle-tree --category=compression

# Create oracle program
bun create price-feed --category=oracles
```

### `bun generate` (Smart Generation)

Automatically generates IDL and TypeScript client based on your current directory:

**From program directory** (e.g., `cd basics/my-counter`):
```bash
bun generate                    # Auto-detects program name
```

**From root directory or explicit**:
```bash
bun generate my-counter                    # Generate for basics/my-counter
bun generate token-vault --category=tokens # Generate for tokens/token-vault
```

### `bun deploy` (Smart Deployment)

Automatically deploys your program based on current directory:

**From program directory**:
```bash
bun run deploy                      # Auto-detects program, deploys to devnet
bun run deploy --network=testnet    # Auto-detects program, deploys to testnet
bun run deploy --network=mainnet    # Auto-detects program, deploys to mainnet
```

**From root directory or explicit**:
```bash
bun run deploy my-counter                          # Deploy basics/my-counter
bun run deploy token-vault --category=tokens      # Deploy tokens/token-vault
bun run deploy my-counter --network=mainnet       # Deploy to mainnet
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

### Build Scripts
- Updates root `package.json` with program-specific scripts:
  - `gen:idl:[program-name]` - Generate IDL using Shank
  - `gen:client:[program-name]` - Generate TypeScript client
  - `test:client:[program-name]` - Run tests

## 🎯 Template Features

The counter template includes:

### State Management
- `Counter` struct with proper bytemuck traits for zero-copy deserialization
- Built-in safety methods (increment, decrement with overflow protection)
- Authority-based access control

### Instructions
- Comprehensive Shank annotations for IDL generation
- Multiple instruction variants (Initialize, Increment, Decrement, SetValue, Reset)
- Proper account documentation

### Error Handling
- Pinocchio error handling patterns
- Input validation
- Overflow/underflow protection

### Testing with Kite
- Uses [Solana Kite](https://solanakite.org) for simplified testing
- **Helius RPC Integration**: Automatically uses high-performance Helius RPC endpoints
- Automatic wallet creation and SOL airdropping
- Streamlined transaction sending and confirmation
- Built on top of @solana/kit for modern Solana development
- Enhanced network reliability and faster transaction confirmation

## 📁 Program Categories

Programs are organized into categories:

- `basics/` - Basic Solana programs
- `tokens/` - Token-related programs
- `compression/` - State compression programs
- `oracles/` - Oracle and data feed programs

## 🔄 Development Workflow Examples

### Option 1: Context-Aware Workflow (Recommended)
```bash
# Create new program
bun create token-staking --category=tokens

# Navigate to program directory
cd tokens/token-staking

# Edit your code...
# vim src/lib.rs

# Generate IDL and client (auto-detects token-staking)
bun generate

# Deploy to devnet (auto-detects token-staking)
bun run deploy

# Test with generated client
bun test

# Deploy to mainnet when ready
bun run deploy --network=mainnet
```

### Option 2: Explicit Commands
```bash
# Create program
bun create user-rewards

# Generate from root directory
bun generate user-rewards

# Deploy from root directory
bun run deploy user-rewards --network=testnet
```

### Option 3: Multiple Programs
```bash
# Create multiple related programs
bun create user-accounts
bun create voting-system
bun create reward-pool

# Work on each one individually
cd basics/user-accounts
bun generate && bun run deploy

cd ../voting-system
bun generate && bun run deploy

cd ../reward-pool
bun generate && bun run deploy
```

## 🛠️ Legacy Script Access

The original scripts are still available in the `scripts/` directory:

```bash
# Create program (legacy)
./scripts/create-program.sh my-program

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

## 🔧 Customizing Templates

To create new templates:

1. Create a new directory in `templates/`
2. Add the necessary Rust source files
3. Use placeholder names (like "counter") that will be replaced
4. Update the `create-program.sh` script to reference your template

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
bun create --help          # Create command help
bun generate --help        # Generate command help  
bun run deploy --help          # Deploy command help
```

## 📚 Integration with Existing Workflow

This template system integrates with your existing scripts:

- `scripts/deploy.sh` - Handles building and deploying any program with Helius RPC
- `scripts/update-program-ids.sh` - Updates program IDs after deployment
- `scripts/generate-clients.js` - Generates TypeScript clients
- `scripts/rpc-config.js` - Centralized RPC endpoint management

The new bun aliases extend these tools to work seamlessly with intelligent context detection and simplified commands.
