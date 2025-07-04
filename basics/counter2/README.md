# Counter2

A Solana program built with Pinocchio.

## Description

TODO: Add description of what this program does.

## Usage

### Building

```bash
cargo build-sbf --manifest-path basics/counter2/Cargo.toml
```

### Deployment

```bash
# Deploy to devnet
./deploy.sh counter2

# Deploy to testnet
./deploy.sh counter2 --network=testnet

# Deploy to mainnet
./deploy.sh counter2 --network=mainnet
```

### Generate Client

```bash
# Generate IDL
npm run gen:idl:counter2

# Generate TypeScript client
npm run gen:client:counter2
```

### Testing

```bash
# Run tests
npm run test:client:counter2
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
2. Build and deploy: `./deploy.sh counter2`
3. Generate client: `npm run gen:client:counter2`
4. Run tests: `npm run test:client:counter2`

## Notes

- This program was created from the Pinocchio template
- Program ID will be updated after first deployment
- Remember to commit changes after successful deployment
