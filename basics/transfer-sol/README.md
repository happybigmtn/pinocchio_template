# Transfer Sol

A Solana program built with Pinocchio.

## Description

TODO: Add description of what this program does.

## Usage

### Building

```bash
cargo build-sbf --manifest-path basics/transfer-sol/Cargo.toml
```

### Deployment

```bash
# Deploy to devnet
./deploy.sh transfer-sol

# Deploy to testnet
./deploy.sh transfer-sol --network=testnet

# Deploy to mainnet
./deploy.sh transfer-sol --network=mainnet
```

### Generate Client

```bash
# Generate IDL
npm run gen:idl:transfer-sol

# Generate TypeScript client
npm run gen:client:transfer-sol
```

### Testing

```bash
# Run tests
npm run test:client:transfer-sol
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
2. Build and deploy: `./deploy.sh transfer-sol`
3. Generate client: `npm run gen:client:transfer-sol`
4. Run tests: `npm run test:client:transfer-sol`

## Notes

- This program was created from the Pinocchio template
- Program ID will be updated after first deployment
- Remember to commit changes after successful deployment
