# Pinocchio Template System

This template system allows you to quickly create new Pinocchio programs with all the necessary boilerplate code, build scripts, and deployment configuration.

## Quick Start

### Option 1: Create and Deploy in One Command

```bash
# Create and deploy a new program called 'my_counter' to devnet
./quick-deploy.sh my_counter

# Create and deploy to a specific category and network
./quick-deploy.sh token_mint --category=tokens --network=testnet
```

### Option 2: Create Program Only

```bash
# Create a new program from template
./create-program.sh my_counter

# Create with specific category
./create-program.sh token_mint --category=tokens

# Then deploy separately
./deploy.sh my_counter
```

## What Gets Created

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

## Template Features

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

## Available Scripts

After creating a program named `my_counter`, these scripts become available:

```bash
# Generate IDL from Rust code
npm run gen:idl:my-counter

# Generate TypeScript client from IDL
npm run gen:client:my-counter

# Run program tests
npm run test:client:my-counter

# Deploy the program
./deploy.sh my_counter

# Or use the program-specific deploy script
cd basics/my_counter && ./deploy.sh
```

## Development Workflow

1. **Create Program**: `./create-program.sh my_program`
2. **Edit Code**: Modify files in `basics/my_program/src/`
3. **Deploy**: `./deploy.sh my_program`
4. **Generate Client**: `npm run gen:client:my-program`
5. **Test**: `npm run test:client:my-program`
6. **Iterate**: Repeat steps 2-5 as needed

## Program Categories

Programs are organized into categories:

- `basics/` - Basic Solana programs
- `tokens/` - Token-related programs
- `compression/` - State compression programs
- `oracles/` - Oracle and data feed programs

## Template Structure

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

## Customizing Templates

To create new templates:

1. Create a new directory in `templates/`
2. Add the necessary Rust source files
3. Use placeholder names (like "counter") that will be replaced
4. Update the `create-program.sh` script to reference your template

## Examples

### Create a Basic Counter
```bash
./quick-deploy.sh my_counter
```

### Create a Token Program
```bash
./create-program.sh token_staking --category=tokens
./deploy.sh token_staking
```

### Create Multiple Programs
```bash
./create-program.sh user_accounts
./create-program.sh voting_system
./create-program.sh reward_pool
```

## Troubleshooting

### Program Already Exists
If you see "Program already exists", either:
- Choose a different name
- Remove the existing program directory
- Use `--force` flag (if implemented)

### Deployment Fails
- Check your Solana CLI configuration: `solana config get`
- Ensure you have sufficient SOL for deployment
- Verify network connectivity

### Client Generation Fails
- Ensure the program builds successfully first
- Check that Shank annotations are correct
- Verify IDL generation works: `npm run gen:idl:program-name`

## RPC Configuration

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

## Integration with Existing Workflow

This template system integrates with your existing scripts:

- `deploy.sh` - Handles building and deploying any program with Helius RPC
- `update-program-ids.sh` - Updates program IDs after deployment
- `scripts/generate-clients.js` - Generates TypeScript clients
- `scripts/rpc-config.js` - Centralized RPC endpoint management

The template system extends these tools to work seamlessly with new programs created from templates.
