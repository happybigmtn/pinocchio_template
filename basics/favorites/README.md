# Favorites

A Solana program built with Pinocchio.

## Description

TODO: Add description of what this program does.

## Usage

### Building

```bash
cargo build-sbf --manifest-path basics/favorites/Cargo.toml
```

### Deployment

```bash
# Deploy to devnet
./deploy.sh favorites

# Deploy to testnet
./deploy.sh favorites --network=testnet

# Deploy to mainnet
./deploy.sh favorites --network=mainnet
```

### Generate Client

```bash
# Generate IDL
npm run gen:idl:favorites

# Generate TypeScript client
npm run gen:client:favorites
```

### Testing

```bash
# Run tests
npm run test:client:favorites
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
2. Build and deploy: `./deploy.sh favorites`
3. Generate client: `npm run gen:client:favorites`
4. Run tests: `npm run test:client:favorites`

## Notes

- This program was created from the Pinocchio template
- Program ID will be updated after first deployment
- Remember to commit changes after successful deployment
