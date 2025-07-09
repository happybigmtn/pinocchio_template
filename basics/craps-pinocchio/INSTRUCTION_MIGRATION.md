# Instruction Consolidation Migration Guide

## Overview
This guide helps transition from the old 32-instruction API to the new consolidated 22-instruction API.

## Instruction Changes

### 1. Cleanup Instructions
**Old:**
```rust
CleanupBetBatch        // discriminator: 15
CleanupOldBetBatch     // discriminator: 16
```

**New:**
```rust
CleanupBetBatch        // discriminator: 13 (use for both cases)
```

### 2. Authority Update Instructions
**Old:**
```rust
UpdateAuthority          // discriminator: 18
UpdateRngAuthority       // discriminator: 19  
UpdateAdminAuthority     // discriminator: 20
UpdateEmergencyAuthority // discriminator: 21
UpdateTreasuryAuthority  // discriminator: 30
```

**New:**
```rust
UpdateAuthority          // discriminator: 15
// Instruction data format:
// [authority_type: u8, new_authority: Pubkey]
// Where authority_type is:
//   0 = System
//   1 = Rng
//   2 = Admin
//   3 = Emergency
//   4 = Treasury
```

### 3. Emergency Operations
**Old:**
```rust
EmergencyShutdown   // discriminator: 23
ResumeOperations    // discriminator: 24
EmergencyPause      // discriminator: 25
ResumeGame          // discriminator: 26
```

**New:**
```rust
EmergencyOperation  // discriminator: 17
// Instruction data format:
// [operation_type: u8]
// Where operation_type is:
//   0 = Shutdown
//   1 = Resume
//   2 = PauseGame
//   3 = ResumeGame
```

### 4. Treasury Operations
**Old:**
```rust
DepositV2                 // discriminator: 4
DepositWithAutoClaimV2    // discriminator: 6
WithdrawV2                // discriminator: 5
WithdrawWithAutoClaimV2   // discriminator: 7
```

**New:**
```rust
Deposit     // discriminator: 4
Withdraw    // discriminator: 5
// Instruction data format:
// [auto_claim: u8, amount: u64]
// Where auto_claim is:
//   0 = false (no auto-claim)
//   1 = true (with auto-claim)
```

## Client Code Examples

### JavaScript/TypeScript
```typescript
// Old way - multiple authority updates
await program.methods.updateRngAuthority(newRngAuthority)
  .accounts({...})
  .rpc();

// New way - single instruction with type
const authorityType = { rng: {} }; // or system, admin, emergency, treasury
await program.methods.updateAuthority(authorityType, newAuthority)
  .accounts({...})
  .rpc();
```

### Rust
```rust
// Old way - emergency shutdown
let ix = EmergencyShutdown {};

// New way - emergency operation
let ix = EmergencyOperation {
    operation: EmergencyOperationType::Shutdown,
};
```

## New Discriminator Mapping

| Instruction                   | Old Discriminator | New Discriminator |
|------------------------------|-------------------|-------------------|
| InitializeSystem             | 0                 | 0                 |
| InitializeCriticalPDAs       | 1                 | 1                 |
| InitializePlayer             | 2                 | 2                 |
| ClosePlayerAccount           | 3                 | 3                 |
| Deposit                      | 4, 6              | 4                 |
| Withdraw                     | 5, 7              | 5                 |
| PlaceBet                     | 8                 | 6                 |
| SecureAutoRoll               | 9                 | 7                 |
| CollectBlockHash             | 10                | 8                 |
| FinalizeRng                  | 11                | 9                 |
| StartBettingPhase            | 12                | 10                |
| SettleRealizableBets         | 13                | 11                |
| ClaimEpochPayoutsUnified     | 14                | 12                |
| CleanupBetBatch              | 15, 16            | 13                |
| CleanupOldEpochOutcome       | 17                | 14                |
| UpdateAuthority              | 18-21, 30         | 15                |
| ExecuteAuthorityTransfer     | 22                | 16                |
| EmergencyOperation           | 23-26             | 17                |
| EnableSecureRng              | 27                | 18                |
| UpdatePlayerTournament       | 28                | 19                |
| ClearPlayerTournament        | 29                | 20                |
| UpdateTreasuryParameters     | 31                | 21                |

## Testing the Migration

1. Update your client code to use the new instruction formats
2. Test authority updates with different authority types
3. Test emergency operations with different operation types
4. Test deposit/withdraw with and without auto-claim
5. Verify cleanup operations work with the single instruction

## Benefits

1. **Reduced Complexity**: 31% fewer instructions to maintain
2. **Smaller Program Size**: Less code duplication
3. **Cleaner API**: More logical grouping of related operations
4. **Future-Proof**: Easier to add new authority types or operations without new instructions