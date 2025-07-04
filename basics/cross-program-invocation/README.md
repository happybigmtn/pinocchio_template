# Cross Program Invocation

A Solana program built with Pinocchio.

## Description

TODO: Add description of what this program does.

## Usage

### Building

```bash
cargo build-sbf --manifest-path basics/cross-program-invocation/Cargo.toml
```

### Deployment

```bash
# Deploy to devnet
./deploy.sh cross-program-invocation

# Deploy to testnet
./deploy.sh cross-program-invocation --network=testnet

# Deploy to mainnet
./deploy.sh cross-program-invocation --network=mainnet
```

### Generate Client

```bash
# Generate IDL
npm run gen:idl:cross-program-invocation

# Generate TypeScript client
npm run gen:client:cross-program-invocation
```

### Testing

```bash
# Run tests
npm run test:client:cross-program-invocation
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
2. Build and deploy: `./deploy.sh cross-program-invocation`
3. Generate client: `npm run gen:client:cross-program-invocation`
4. Run tests: `npm run test:client:cross-program-invocation`

## Notes

- This program was created from the Pinocchio template
- Program ID will be updated after first deployment
- Remember to commit changes after successful deployment
