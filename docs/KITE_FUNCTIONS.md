# Kite Functions Demonstrated in Test Templates

This document lists all the Solana Kite functions that are demonstrated in our comprehensive test templates. The test templates serve as both functional tests and educational examples for using Kite.

## 🔍 Overview

Our test templates demonstrate **17 core Kite functions** across 4 main categories:

1. **Wallet Management** (3 functions)
2. **SOL Management** (3 functions) 
3. **Token Management** (6 functions)
4. **Transaction & Utility** (5 functions)

## 🔑 Wallet Management Functions

### 1. `createWallet(options?)` 
Creates a new Solana wallet with optional configurations.

**Demonstrated:**
- Basic wallet creation
- Custom wallet with airdrop amount
- Wallet with address prefix/suffix

```typescript
const basicWallet = await kite.createWallet();
const customWallet = await kite.createWallet({ 
  airdropAmount: lamports(2_000_000_000n), // 2 SOL
  prefix: 'COOL',
  suffix: 'TEST'
});
```

### 2. `createWallets(count, options?)` 
Creates multiple wallets at once.

**Demonstrated:**
- Creating multiple wallets with airdrop amounts

```typescript
const multipleWallets = await kite.createWallets(3, {
  airdropAmount: lamports(500_000_000n) // 0.5 SOL each
});
```

### 3. Wallet Loading Functions
- `loadWalletFromFile(path)` - Load wallet from file
- `loadWalletFromEnvironment(envVar)` - Load wallet from environment variable

*Note: These are referenced in the documentation but not actively demonstrated in the test template since they require external files/env vars.*

## 💰 SOL Management Functions

### 3. `getLamportBalance(address, commitment?)` 
Gets the SOL balance of an account in lamports.

**Demonstrated:**
- Checking balances before and after transfers
- Balance monitoring across multiple wallets

```typescript
const balance = await kite.getLamportBalance(sender.address);
console.log('Balance:', Number(balance) / 1_000_000_000, 'SOL');
```

### 4. `airdropIfRequired(address, airdropAmount, minimumBalance)` 
Conditionally airdrops SOL if balance is below threshold.

**Demonstrated:**
- Conditional airdropping based on minimum balance
- Handling cases where airdrop is/isn't needed

```typescript
const airdropSig = await kite.airdropIfRequired(
  receiver.address,
  lamports(1_500_000_000n), // 1.5 SOL
  lamports(1_000_000_000n)  // 1 SOL minimum
);
```

### 5. `transferLamports(options)` 
Transfers SOL between wallets.

**Demonstrated:**
- Basic SOL transfers
- Advanced options (skipPreflight, retries)
- Balance verification after transfer

```typescript
const transferSig = await kite.transferLamports({
  source: sender,
  destination: receiver.address,
  amount: lamports(250_000_000n), // 0.25 SOL
  skipPreflight: false,
  maximumClientSideRetries: 3
});
```

## 🪙 Token Management Functions

### 6. `createTokenMint(options)` 
Creates a new SPL token mint with metadata.

**Demonstrated:**
- Token creation with full metadata
- Custom decimals, name, symbol, URI
- Additional metadata fields

```typescript
const mintAddress = await kite.createTokenMint({
  mintAuthority,
  decimals: 9,
  name: 'Test Token',
  symbol: 'TEST',
  uri: 'https://example.com/token.json',
  additionalMetadata: {
    description: 'A test token created with Kite',
    category: 'utility'
  }
});
```

### 7. `getMint(mintAddress)` 
Gets token mint information.

**Demonstrated:**
- Retrieving mint decimals and supply
- Verifying mint creation

```typescript
const mintInfo = await kite.getMint(mintAddress);
console.log('Decimals:', mintInfo.decimals, 'Supply:', mintInfo.supply);
```

### 8. `getTokenAccountAddress(wallet, mint, useTokenExtensions?)` 
Gets the associated token account address.

**Demonstrated:**
- Getting token account addresses for multiple wallets
- Standard token program usage

```typescript
const tokenAccount = await kite.getTokenAccountAddress(
  wallet.address,
  mintAddress
);
```

### 9. `mintTokens(mintAddress, authority, amount, destination)` 
Mints tokens to an account.

**Demonstrated:**
- Minting tokens with proper decimals
- Authority-based minting

```typescript
const mintAmount = 1000n * 10n ** 9n; // 1000 tokens with 9 decimals
const mintSig = await kite.mintTokens(
  mintAddress,
  mintAuthority,
  mintAmount,
  mintAuthority.address
);
```

### 10. `getTokenAccountBalance(tokenAccount)` 
Gets token account balance.

**Demonstrated:**
- Balance checking before and after operations
- Proper decimal handling for display

```typescript
const balance = await kite.getTokenAccountBalance(tokenAccount);
console.log('Balance:', Number(balance.amount) / 10**9, 'tokens');
```

### 11. `transferTokens(options)` 
Transfers tokens between accounts.

**Demonstrated:**
- Token transfers with retry options
- Balance verification after transfer

```typescript
const tokenTransferSig = await kite.transferTokens({
  sender: mintAuthority,
  destination: recipient.address,
  mintAddress,
  amount: transferAmount,
  maximumClientSideRetries: 3
});
```

### 12. `checkTokenAccountIsClosed(tokenAccount)` 
Checks if a token account is closed.

**Demonstrated:**
- Verifying account status for multiple accounts

```typescript
const isClosed = await kite.checkTokenAccountIsClosed(tokenAccount);
console.log('Account closed:', isClosed);
```

## ⚙️ Transaction & Utility Functions

### 13. `sendTransactionFromInstructions(options)` 
Sends a transaction containing multiple instructions.

**Demonstrated:**
- Multi-instruction transactions
- Advanced transaction options
- Proper fee payer configuration

```typescript
const multiInstructionSig = await kite.sendTransactionFromInstructions({
  feePayer: wallet,
  instructions: [instruction1, instruction2],
  commitment: 'confirmed',
  skipPreflight: false,
  maximumClientSideRetries: 3
});
```

### 14. `getRecentSignatureConfirmation(signature)` 
Checks if a transaction is confirmed.

**Demonstrated:**
- Transaction confirmation checking
- Post-transaction verification

```typescript
const isConfirmed = await kite.getRecentSignatureConfirmation(signature);
console.log('Transaction confirmed:', isConfirmed);
```

### 15. `getLogs(signature)` 
Gets transaction logs.

**Demonstrated:**
- Retrieving and displaying transaction logs
- Log analysis for debugging

```typescript
const logs = await kite.getLogs(signature);
console.log('Transaction logs:', logs.slice(0, 3));
```

### 16. `getPDAAndBump(seeds, programId)` 
Gets Program Derived Address and bump seed.

**Demonstrated:**
- PDA generation with custom seeds
- Bump seed retrieval

```typescript
const seeds = [Buffer.from('test'), wallet.address.toBytes()];
const [pda, bump] = await kite.getPDAAndBump(seeds, programId);
```

### 17. `getExplorerLink(type, id)` 
Gets Solana Explorer links.

**Demonstrated:**
- Address, transaction, and block explorer links
- Multiple link types

```typescript
const addressLink = kite.getExplorerLink('address', wallet.address);
const transactionLink = kite.getExplorerLink('transaction', signature);
const blockLink = kite.getExplorerLink('block', '12345');
```

## 🏗️ Test Structure

Our test templates organize demonstrations into logical groups:

### Test 1: Wallet Management Functions
- Basic and advanced wallet creation
- Multiple wallet creation
- Address customization

### Test 2: SOL Balance and Transfer Functions  
- Balance checking
- Conditional airdropping
- SOL transfers with options

### Test 3: Token Functions
- Complete token lifecycle
- Mint creation → minting → transfers → balance checking
- Account status verification

### Test 4: Transaction and Utility Functions
- Multi-instruction transactions
- Transaction confirmation and logging
- PDA generation and explorer links

### Test 5: Program-Specific Functionality
- Template for adding custom program tests
- Integration with generated clients

## 📚 Educational Value

Each function demonstration includes:

✅ **Practical Examples** - Real-world usage patterns  
✅ **Error Handling** - Proper try/catch blocks  
✅ **Console Output** - Clear logging for learning  
✅ **Parameter Explanations** - Comments explaining options  
✅ **Best Practices** - Recommended patterns and timeouts  

## 🔗 References

- **Kite Documentation**: https://github.com/helius-labs/kite-og
- **Solana Kit**: https://github.com/anza-xyz/kit
- **Test Template Location**: `{program}/tests/{program}.test.ts`

## 🎯 Usage

When you create a new program with our template system:

```bash
bun run create my-program --template=counter
```

The generated test file will include demonstrations of all these functions, serving as both a comprehensive test suite and an educational resource for learning Kite.

---

*This comprehensive test template makes learning Solana development with Kite much easier by providing working examples of every major function.*
