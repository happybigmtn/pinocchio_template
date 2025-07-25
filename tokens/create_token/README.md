# Create Token

A Solana program built with Pinocchio.

## Description

TODO: Add description of what this program does.

## Usage

### Building

```bash
cargo build-sbf --manifest-path tokens/create-token/Cargo.toml
```

### Deployment

```bash
# Deploy to devnet
./deploy.sh create-token

# Deploy to testnet
./deploy.sh create-token --network=testnet

# Deploy to mainnet
./deploy.sh create-token --network=mainnet
```

### Generate Client

```bash
# Generate IDL
npm run gen:idl:create-token

# Generate TypeScript client
npm run gen:client:create-token
```

### Testing

```bash
# Run tests
npm run test:client:create-token
```

## Program Structure

- `src/lib.rs` - Main program entry point
- `src/processor.rs` - Instruction processing logic
- `src/instructions/` - Instruction definitions
- `src/state/` - Account state definitions
- `src/constants.rs` - Program constants
- `tests/` - Test files

## Development

1. Modify the Rust source code in `src/`
2. Build and deploy: `./deploy.sh create-token`
3. Generate client: `npm run gen:client:create-token`
4. Run tests: `npm run test:client:create-token`

## Notes

- This program was created from the Pinocchio template
- Program ID will be updated after first deployment
- Remember to commit changes after successful deployment
