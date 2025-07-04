# Deployment Guide

This guide explains how to deploy Pinocchio programs and automatically update program IDs.

## Current Program Status

Based on the current configuration:

- **Account Data Program**: `EAUvJAw61MTaJbyV4tqFB4dEZuYHdYrtpGQ35hDsQ6Dw`
- **Close Account Program**: `H9ZpziEUkrhakmLKaFXeokJFhTFm69jJ8aVSso43PopB`

## Quick Start

### Deploy a Program

```bash
# Deploy account_data program to devnet
./deploy.sh

# Deploy a specific program
./deploy.sh account_data

# Deploy to testnet
./deploy.sh account_data --network=testnet
```

### Update Program IDs Only

If you've already deployed and just need to update the program IDs:

```bash
# Update account_data program IDs
./update-program-ids.sh

# Update specific program
./update-program-ids.sh close_account
```

## Scripts Overview

### `deploy.sh` - Complete Deployment Pipeline

This script handles the entire deployment process:

1. ✅ Checks prerequisites (Solana CLI, Cargo, Bun)
2. 🌐 Sets the target Solana cluster
3. 💰 Checks wallet balance
4. 🔨 Builds the program using `cargo build-sbf`
5. 🚀 Deploys the program to Solana
6. 🔄 Updates program IDs in source code
7. 🧪 Runs tests with updated client
8. 📊 Shows deployment summary

**Usage:**
```bash
./deploy.sh [program_name] [--network=NETWORK]
```

**Examples:**
```bash
./deploy.sh                                    # Deploy account_data to devnet
./deploy.sh close_account                      # Deploy close_account to devnet  
./deploy.sh account_data --network=testnet     # Deploy to testnet
./deploy.sh account_data --network=mainnet     # Deploy to mainnet
```

### `update-program-ids.sh` - Program ID Synchronization

This script updates program IDs after deployment:

1. 🔍 Finds the program ID from keypair or deployed program
2. 📝 Updates Rust `lib.rs` file (`declare_id!` and `#[shank(id)]`)
3. ⚙️ Updates Codama configuration (`codama-node.ts`)
4. 🔄 Regenerates TypeScript client code

**Usage:**
```bash
./update-program-ids.sh [program_name]
```

## Manual Process

If you prefer to do things manually:

### 1. Build the Program

```bash
cargo build-sbf --manifest-path basics/account_data/Cargo.toml
```

### 2. Deploy to Solana

```bash
# First deployment (creates new program)
solana program deploy target/deploy/account_data.so

# Update existing program
solana program deploy target/deploy/account_data.so --program-id target/deploy/account_data-keypair.json
```

### 3. Get the Program ID

```bash
solana-keygen pubkey target/deploy/account_data-keypair.json
```

### 4. Update Source Files

Update the program ID in:
- `basics/account_data/src/lib.rs` - `declare_id!` macro
- `basics/account_data/codama-node.ts` - `publicKey` field

### 5. Regenerate Client

```bash
cd basics/account_data
bun generate-client.ts
```

## Networks

### Devnet (Default)
- Good for development and testing
- Free SOL available from faucet
- Reset periodically

### Testnet  
- Stable testing environment
- Free SOL available from faucet
- Persistent

### Mainnet
- Production environment
- Real SOL required
- Permanent

## Prerequisites

Make sure you have these tools installed:

- **Solana CLI**: `sh -c "$(curl -sSfL https://release.solana.com/v1.18.8/install)"`
- **Rust**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Bun**: `curl -fsSL https://bun.sh/install | bash`

## Troubleshooting

### "Insufficient funds"
Get SOL from the faucet:
- Devnet/Testnet: https://faucet.solana.com
- Or use: `solana airdrop 2`

### "Program already exists"
Use the existing keypair:
```bash
solana program deploy target/deploy/account_data.so --program-id target/deploy/account_data-keypair.json
```

### "RPC URL not set"
Set your cluster:
```bash
solana config set --url devnet
```

### Program ID Mismatch
Run the update script:
```bash
./update-program-ids.sh account_data
```

## Best Practices

1. **Always test on devnet first** before deploying to mainnet
2. **Backup your keypairs** - store them securely
3. **Version your deployments** - tag your releases
4. **Run tests** after updating program IDs
5. **Commit changes** after successful deployment

## Program Structure

```
basics/
├── account_data/
│   ├── src/
│   │   ├── lib.rs              # Contains declare_id! macro
│   │   └── ...
│   ├── codama-node.ts          # Contains publicKey configuration  
│   └── generate-client.ts      # Client generation script
└── ...

clients/
├── accountData/                # Generated TypeScript client
│   ├── programs/
│   │   └── accountData.ts      # Contains ACCOUNT_DATA_PROGRAM_ADDRESS
│   └── ...
└── ...

target/
└── deploy/
    ├── account_data.so         # Compiled program binary
    └── account_data-keypair.json # Program keypair
```

## Next Steps

After successful deployment:

1. Test your program with the updated client
2. Commit your changes to version control
3. Update any documentation with new program IDs
4. Deploy to other networks as needed
