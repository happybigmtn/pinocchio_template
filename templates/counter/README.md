# Counter

A Solana program built with Pinocchio.

## Description

TODO: Add description of what this program does.

## Usage

### Building

```bash
cargo build-sbf --manifest-path basics/counter/Cargo.toml
```

### Deployment

```bash
# Deploy to devnet
./deploy.sh counter

# Deploy to testnet
./deploy.sh counter --network=testnet

# Deploy to mainnet
./deploy.sh counter --network=mainnet
```

### Generate Client

```bash
# Generate IDL
npm run gen:idl:counter

# Generate TypeScript client
npm run gen:client:counter
```

### Testing

```bash
# Run tests
npm run test:client:counter
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
2. Build and deploy: `./deploy.sh counter`
3. Generate client: `npm run gen:client:counter`
4. Run tests: `npm run test:client:counter`

## Notes

- This program was created from the Pinocchio template
- Program ID will be updated after first deployment
- Remember to commit changes after successful deployment
