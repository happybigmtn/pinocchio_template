# Craps Anchor to Pinocchio Migration Guide

## A Feynman-Style Tutorial for Complete Migration

### Table of Contents

1. [Quick Start: Your First Hour with Pinocchio](#quick-start)
2. [Introduction: Why Migrate to Pinocchio?](#introduction)
3. [Chapter 1: Understanding the Fundamental Differences](#chapter-1)
4. [Chapter 2: Account Structure Migration](#chapter-2)
5. [Chapter 3: Instruction Processing Migration](#chapter-3)
6. [Chapter 4: State Management and Bit-Packing](#chapter-4)
7. [Chapter 5: RNG System Migration](#chapter-5)
8. [Chapter 6: Treasury and Token Operations](#chapter-6)
9. [Chapter 7: Error Handling and Events](#chapter-7)
10. [Chapter 8: Testing and Deployment](#chapter-8)
11. [Chapter 9: TUI Migration](#chapter-9)
12. [Troubleshooting: Common Migration Issues](#troubleshooting)
13. [Appendix: Complete Code Reference](#appendix)

---

## Quick Start: Your First Hour with Pinocchio

Want to get started quickly? Here's what you can accomplish in your first hour:

### Minute 0-10: Setup

```bash
# Clone the Pinocchio template
git clone https://github.com/exo-tech-xyz/pinocchio-project
cd pinocchio-project

# Install dependencies
npm install
cargo build

# Create your program
./scripts/create-program.sh craps-game
```

### Minute 10-20: Your First Account

**Anchor Way:**
```rust
#[account]
pub struct Player {
    pub balance: u64,
    pub games_played: u32,
}
```

**Pinocchio Way:**
```rust
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Player {
    pub balance: [u8; 8],      // u64 as bytes
    pub games_played: [u8; 4],  // u32 as bytes
    pub _padding: [u8; 20],     // Align to 32 bytes
}

impl Player {
    pub const LEN: usize = 32;
    
    pub fn get_balance(&self) -> u64 {
        u64::from_le_bytes(self.balance)
    }
}
```

### Minute 20-30: Your First Instruction

**Anchor Way:**
```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.player.balance = 1000;
    Ok(())
}
```

**Pinocchio Way:**
```rust
pub fn initialize_handler(accounts: &[AccountInfo]) -> ProgramResult {
    let player_account = &accounts[0];
    
    // Manual validation
    if !player_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Get mutable data
    let mut data = player_account.data.borrow_mut();
    let player = Player::cast_mut(&mut data);
    
    // Set balance
    player.balance = 1000u64.to_le_bytes();
    
    Ok(())
}
```

### Minute 30-40: Build and Test

```bash
# Build your program
cargo build-sbf

# Run tests
cargo test

# Deploy to devnet
solana program deploy target/deploy/craps_pinocchio.so --url devnet
```

### Minute 40-50: Common Patterns

**Pattern 1: Reading Accounts**
```rust
// Always borrow immutably for reads
let data = account.data.borrow();
let state = MyState::cast(&data);
let value = state.get_value();
```

**Pattern 2: Writing Accounts**
```rust
// Borrow mutably for writes
let mut data = account.data.borrow_mut();
let state = MyState::cast_mut(&mut data);
state.set_value(42);
```

**Pattern 3: PDA Validation**
```rust
let (expected_pda, bump) = Pubkey::find_program_address(
    &[b"seed", user.as_ref()],
    &crate::ID
);
if account.key() != &expected_pda {
    return Err(ProgramError::InvalidSeeds);
}
```

### Minute 50-60: Your First Transaction

```typescript
// Client-side
const ix = createInitializeInstruction({
    player: playerPDA,
    payer: wallet.publicKey,
    systemProgram: SystemProgram.programId,
});

const tx = new Transaction().add(ix);
await sendAndConfirmTransaction(connection, tx, [wallet]);
```

**Congratulations!** You've just built your first Pinocchio program. The key insight? You're now in control of every byte, every check, every operation.

---

## Introduction: Why Migrate to Pinocchio?

Imagine you're Richard Feynman, and someone asks you: "Why should I migrate from Anchor to Pinocchio?"

You'd probably answer: "Well, let me tell you about efficiency. When you write a Solana program, every byte matters, every compute unit counts. Anchor is like driving a comfortable sedan - it gets you there safely with all the conveniences. Pinocchio is like a Formula 1 car - stripped down to essentials, incredibly fast, but you need to know exactly what you're doing."

### Key Benefits of Pinocchio:

1. **Zero-Copy Deserialization**: No allocation overhead
2. **No Standard Library**: Smaller program size
3. **Direct Memory Access**: Faster execution
4. **Minimal Dependencies**: Better security
5. **Lower Compute Units**: More efficient programs

### What We're Migrating:

- **64 Bet Types**: All craps betting options
- **Secure RNG**: Commit-reveal pattern
- **Treasury System**: Token management
- **Tournament Integration**: Via CPI
- **Mobile TUI**: Complete user interface

---

## Chapter 1: Understanding the Fundamental Differences

Let's start with a simple analogy. If Anchor is like cooking with a fully equipped kitchen, Pinocchio is like cooking over a campfire. You can make the same meal, but you need different techniques.

### 1.1 The Philosophy Shift

**Anchor Philosophy**: "Let the framework handle the details"
```rust
#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,
    // Anchor handles deserialization
}
```

**Pinocchio Philosophy**: "You control every byte"
```rust
pub struct PlaceBetAccounts<'a> {
    pub player: &'a AccountInfo,
    // You handle deserialization
}
```

### 1.2 Memory Management

In Anchor, you think in terms of objects. In Pinocchio, you think in terms of bytes.

**Anchor**: 
```rust
player.balance += amount;  // Simple!
```

**Pinocchio**:
```rust
let balance = u64::from_le_bytes(player_data[0..8].try_into()?);
let new_balance = balance.checked_add(amount)?;
player_data[0..8].copy_from_slice(&new_balance.to_le_bytes());
```

### 1.3 The Mental Model

Think of it this way:
- Anchor = High-level language (Python)
- Pinocchio = Assembly language

Both can solve the same problems, but Pinocchio gives you direct control over the machine.

---

## Chapter 2: Account Structure Migration

Now, let's migrate our account structures. This is like translating a recipe from metric to imperial - same ingredients, different measurements.

### 2.1 GlobalGameState Migration

**Original Anchor Structure**:
```rust
#[account]
pub struct GlobalGameState {
    pub current_epoch: u64,
    pub dice1: u8,
    pub dice2: u8,
    pub point: u8,
    pub phase: GamePhase,
    pub rng_authority: Pubkey,
    pub admin_authority: Pubkey,
    pub emergency_authority: Pubkey,
    pub treasury: Pubkey,
    pub token_mint: Pubkey,
    pub game_paused: bool,
    pub operations_halted: bool,
}
```

**Pinocchio Migration**:
```rust
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct GlobalGameState {
    pub current_epoch: [u8; 8],        // u64 as bytes
    pub dice1: u8,
    pub dice2: u8,
    pub point: u8,
    pub phase: u8,                     // Enum as u8
    pub rng_authority: [u8; 32],       // Pubkey as bytes
    pub admin_authority: [u8; 32],
    pub emergency_authority: [u8; 32],
    pub treasury: [u8; 32],
    pub token_mint: [u8; 32],
    pub game_paused: u8,               // bool as u8
    pub operations_halted: u8,
    pub _padding: [u8; 12],            // Alignment padding
}

impl GlobalGameState {
    pub const LEN: usize = 8 + 1 + 1 + 1 + 1 + 32 + 32 + 32 + 32 + 32 + 1 + 1 + 12;
    
    pub fn current_epoch(&self) -> u64 {
        u64::from_le_bytes(self.current_epoch)
    }
    
    pub fn set_current_epoch(&mut self, value: u64) {
        self.current_epoch = value.to_le_bytes();
    }
    
    pub fn rng_authority(&self) -> Pubkey {
        Pubkey::from(self.rng_authority)
    }
    
    pub fn set_rng_authority(&mut self, key: &Pubkey) {
        self.rng_authority = key.to_bytes();
    }
}
```

### 2.2 The Bit-Packed BetBatch

This is where things get interesting. Our bet batch uses bit-packing for efficiency.

**Original Anchor**:
```rust
pub struct BetBatch {
    pub player: Pubkey,
    pub epoch: u64,
    pub bets: Vec<u16>,  // Dynamic array
    pub resolved_mask: u16,
    pub realizable_mask: u16,
}
```

**Pinocchio Migration**:
```rust
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct BetBatch {
    pub player: [u8; 32],
    pub epoch: [u8; 8],
    pub bet_count: u8,
    pub resolved_mask: [u8; 2],    // u16 as bytes
    pub realizable_mask: [u8; 2],
    pub settled_mask: [u8; 2],
    pub bets: [u8; 32],            // 16 * u16 packed
    pub _padding: [u8; 13],
}

impl BetBatch {
    pub const LEN: usize = 32 + 8 + 1 + 2 + 2 + 2 + 32 + 13;
    pub const MAX_BETS: usize = 16;
    
    pub fn get_bet(&self, index: usize) -> Option<u16> {
        if index >= self.bet_count as usize {
            return None;
        }
        let start = index * 2;
        let bytes = [self.bets[start], self.bets[start + 1]];
        Some(u16::from_le_bytes(bytes))
    }
    
    pub fn set_bet(&mut self, index: usize, bet: u16) -> Result<(), ProgramError> {
        if index >= Self::MAX_BETS {
            return Err(ProgramError::InvalidArgument);
        }
        let start = index * 2;
        self.bets[start..start + 2].copy_from_slice(&bet.to_le_bytes());
        Ok(())
    }
}
```

### 2.3 Understanding Byte Alignment

Why all the padding? It's like parking cars in a lot - you need everything aligned properly for efficient access.

```
Memory Layout (64-byte aligned):
┌──────────────────────────────────┐
│ player (32 bytes)                │
├──────────────────────────────────┤
│ epoch (8 bytes)                  │
├──────────────────────────────────┤
│ bet_count (1) │ masks (6)        │
├──────────────────────────────────┤
│ bets array (32 bytes)            │
├──────────────────────────────────┤
│ padding (13 bytes)               │
└──────────────────────────────────┘
Total: 64 bytes (cache-line aligned)
```

---

## Chapter 3: Instruction Processing Migration

Instructions are like recipes - in Anchor, you get a cookbook; in Pinocchio, you're writing on index cards.

### 3.1 Instruction Enum Migration

**Anchor**:
```rust
#[derive(Accounts)]
#[instruction(bet_type: u8, amount: u64)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Account<'info, ScalablePlayerState>,
    #[account(mut)]
    pub bet_batch: Account<'info, BetBatch>,
}
```

**Pinocchio**:
```rust
#[derive(ShankInstruction)]
#[repr(u8)]
pub enum CrapsInstruction {
    #[account(0, writable, name = "player")]
    #[account(1, writable, name = "bet_batch")]
    #[account(2, writable, name = "global_state")]
    #[account(3, name = "treasury")]
    #[account(4, name = "clock")]
    PlaceBet = 0,
    
    // ... other instructions
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PlaceBetData {
    pub bet_type: u8,
    pub amount: [u8; 8],  // u64 as bytes
}
```

### 3.2 Processor Implementation

**The Main Router**:
```rust
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id != &crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    
    match CrapsInstruction::try_from(discriminator)? {
        CrapsInstruction::PlaceBet => {
            PlaceBet::try_from((accounts, data))?.handler()
        }
        // Route other instructions...
    }
}
```

### 3.3 Instruction Handler Pattern

```rust
pub struct PlaceBet<'a> {
    pub accounts: PlaceBetAccounts<'a>,
    pub data: PlaceBetData,
}

impl<'a> PlaceBet<'a> {
    pub fn handler(&mut self) -> ProgramResult {
        // 1. Validate accounts
        self.validate_accounts()?;
        
        // 2. Deserialize state
        let mut player_data = self.accounts.player.data.borrow_mut();
        let mut player = ScalablePlayerState::from_bytes_mut(&mut player_data)?;
        
        // 3. Perform business logic
        let amount = u64::from_le_bytes(self.data.amount);
        player.deduct_balance(amount)?;
        
        // 4. Update state
        // State is automatically persisted due to mutable borrow
        
        Ok(())
    }
}
```

---

## Chapter 4: State Management and Bit-Packing

Now we get to the fun part - bit manipulation. It's like playing Tetris with data.

### 4.1 Bet Encoding System

Our bet encoding packs type, amount, and metadata into 16 bits:

```
Bit Layout (16 bits total):
┌─────────┬──────────┬──────────┐
│ Type    │ Amount   │ Flags    │
│ (6 bits)│ (10 bits)│ (0 bits) │
└─────────┴──────────┴──────────┘
```

**Implementation**:
```rust
pub mod bet_encoding {
    pub fn encode_bet(bet_type: u8, amount: u64) -> Result<u16, ProgramError> {
        if bet_type >= 64 {
            return Err(ProgramError::InvalidArgument);
        }
        
        let encoded_amount = encode_amount(amount)?;
        Ok(((bet_type as u16) << 10) | encoded_amount)
    }
    
    pub fn decode_bet(encoded: u16) -> (u8, u64) {
        let bet_type = (encoded >> 10) as u8;
        let amount_bits = encoded & 0x3FF;
        let amount = decode_amount(amount_bits);
        (bet_type, amount)
    }
    
    // Non-linear amount encoding for common bet sizes
    fn encode_amount(amount: u64) -> Result<u16, ProgramError> {
        match amount {
            0 => Ok(0),
            1..=100 => Ok(amount as u16),
            101..=1000 => Ok(100 + ((amount - 100) / 10) as u16),
            1001..=10000 => Ok(190 + ((amount - 1000) / 100) as u16),
            10001..=100000 => Ok(280 + ((amount - 10000) / 1000) as u16),
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}
```

### 4.2 Bet Resolution State Machine

Think of bet resolution like a conveyor belt in a factory:

```
States: Placed → Resolved → Realizable → Settled
         ↓         ↓           ↓            ↓
       Active    Win/Loss    Claimable   Claimed
```

**Bit Mask Operations**:
```rust
impl BetBatch {
    pub fn mark_resolved(&mut self, index: usize) {
        let mask = 1u16 << index;
        let current = u16::from_le_bytes(self.resolved_mask);
        self.resolved_mask = (current | mask).to_le_bytes();
    }
    
    pub fn is_resolved(&self, index: usize) -> bool {
        let mask = 1u16 << index;
        let current = u16::from_le_bytes(self.resolved_mask);
        (current & mask) != 0
    }
    
    pub fn process_resolution(&mut self, index: usize, won: bool) {
        self.mark_resolved(index);
        if won {
            self.mark_realizable(index);
        }
    }
}
```

### 4.3 Multi-Roll Bet Tracking

For bets that span multiple rolls (like "Fire" bets), we use additional state:

```rust
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct BonusState {
    pub player: [u8; 32],
    pub epoch_started: [u8; 8],
    
    // Bit fields for Small/Tall/All tracking
    pub small_hits: u8,     // Tracks 2,3,4,5,6
    pub tall_hits: u8,      // Tracks 8,9,10,11,12
    
    // Fire bet progress
    pub fire_points_made: u8,
    pub fire_points_mask: [u8; 2], // Which points completed
    
    // Repeater tracking
    pub repeater_counts: [u8; 11], // Count for each number
    pub repeater_targets: [u8; 11], // Target for each
}

impl BonusState {
    pub fn update_small_tall(&mut self, roll: u8) {
        match roll {
            2..=6 => self.small_hits |= 1 << (roll - 2),
            8..=12 => self.tall_hits |= 1 << (roll - 8),
            _ => {}
        }
    }
    
    pub fn check_small_complete(&self) -> bool {
        self.small_hits == 0b11111 // All 5 numbers hit
    }
}
```

---

## Chapter 5: RNG System Migration

The RNG system is like a security protocol at a casino - multiple checks, verifiable, and tamper-proof.

### 5.1 Commit-Reveal Pattern

**Phase 1: Betting Phase**
```rust
pub fn start_betting_phase(ctx: Context<StartBetting>) -> Result<()> {
    let rng_state = &mut ctx.accounts.rng_state;
    
    // Record current slot
    let clock = Clock::get()?;
    rng_state.slot_started = clock.slot.to_le_bytes();
    rng_state.phase = RngPhase::Betting as u8;
    
    // Clear previous entropy
    rng_state.block_hashes.fill(0);
    rng_state.hash_count = 0;
    
    Ok(())
}
```

**Phase 2: Collection Phase**
```rust
pub fn collect_block_hash(ctx: Context<CollectHash>) -> Result<()> {
    let rng_state = &mut ctx.accounts.rng_state;
    let slot_hashes = &ctx.accounts.slot_hashes;
    
    // Verify we're in collection phase
    if rng_state.phase != RngPhase::Collecting as u8 {
        return Err(ErrorCode::WrongRngPhase.into());
    }
    
    // Get oldest available hash
    let hash_data = slot_hashes.data.borrow();
    let hash = get_oldest_slot_hash(&hash_data)?;
    
    // Store hash
    let index = rng_state.hash_count as usize;
    rng_state.block_hashes[index].copy_from_slice(&hash);
    rng_state.hash_count += 1;
    
    Ok(())
}
```

**Phase 3: Finalization**
```rust
pub fn finalize_rng(ctx: Context<FinalizeRng>) -> Result<()> {
    let rng_state = &ctx.accounts.rng_state;
    
    // Combine all collected entropy
    let mut hasher = Sha256::new();
    for i in 0..rng_state.hash_count {
        hasher.update(&rng_state.block_hashes[i as usize]);
    }
    
    let hash = hasher.finalize();
    
    // Generate dice using rejection sampling
    let (dice1, dice2) = generate_fair_dice(&hash)?;
    
    // Update game state
    let game_state = &mut ctx.accounts.game_state;
    game_state.dice1 = dice1;
    game_state.dice2 = dice2;
    
    Ok(())
}

fn generate_fair_dice(entropy: &[u8; 32]) -> Result<(u8, u8)> {
    // Rejection sampling for uniform distribution
    let mut index = 0;
    
    loop {
        if index + 1 >= entropy.len() {
            return Err(ErrorCode::InsufficientEntropy.into());
        }
        
        let val1 = entropy[index] % 8;
        let val2 = entropy[index + 1] % 8;
        
        if val1 < 6 && val2 < 6 {
            return Ok((val1 + 1, val2 + 1));
        }
        
        index += 2;
    }
}
```

### 5.2 Security Considerations

The RNG system prevents several attack vectors:

1. **Prediction**: Can't predict future hashes
2. **Manipulation**: Authority separation prevents rigging
3. **Timing**: Fixed collection window prevents gaming
4. **Verification**: All inputs are on-chain and auditable

---

## Chapter 6: Treasury and Token Operations

Token operations in Pinocchio are like handling money in a bank - every transfer needs proper authorization and accounting.

### 6.1 Treasury Account Structure

```rust
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Treasury {
    pub authority: [u8; 32],
    pub token_mint: [u8; 32],
    pub vault: [u8; 32],
    pub total_deposits: [u8; 8],
    pub total_withdrawals: [u8; 8],
    pub total_payouts: [u8; 8],
    pub payout_limit_per_tx: [u8; 8],
    pub emergency_shutdown: u8,
    pub _padding: [u8; 7],
}
```

### 6.2 Token Transfer Operations

**Deposit Implementation**:
```rust
pub fn deposit_v2(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Transfer tokens from player to treasury
    pinocchio_token::instructions::Transfer {
        from: ctx.accounts.player_token_account,
        to: ctx.accounts.treasury_vault,
        amount,
        authority: ctx.accounts.player,
    }.invoke()?;
    
    // Update player balance
    let mut player = ctx.accounts.player_state;
    player.add_balance(amount)?;
    
    // Update treasury stats
    let mut treasury = ctx.accounts.treasury;
    treasury.add_deposit(amount)?;
    
    Ok(())
}
```

### 6.3 PDA Token Accounts

Creating PDA token accounts requires careful seed management:

```rust
pub fn create_player_token_account(ctx: Context<CreatePlayerToken>) -> Result<()> {
    let player_key = ctx.accounts.player.key();
    let (pda, bump) = Pubkey::find_program_address(
        &[b"player_token", player_key.as_ref()],
        &crate::ID
    );
    
    // Create account
    let seeds = &[
        b"player_token",
        player_key.as_ref(),
        &[bump],
    ];
    
    pinocchio_token::instructions::InitializeAccount3 {
        account: ctx.accounts.player_token_account,
        mint: ctx.accounts.token_mint,
        owner: &pda,
    }.invoke_signed(&[seeds])?;
    
    Ok(())
}
```

---

## Chapter 7: Error Handling and Events

In Pinocchio, we handle errors like a detective - every error tells a story.

### 7.1 Error Code System

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrapsError {
    // Betting errors (6000-6099)
    InvalidBetType = 6000,
    InvalidBetAmount = 6001,
    BettingClosed = 6002,
    BetLimitExceeded = 6003,
    
    // Game state errors (6100-6199)
    InvalidGamePhase = 6100,
    InvalidDiceRoll = 6101,
    GamePaused = 6102,
    
    // Player errors (6200-6299)
    InsufficientBalance = 6200,
    PlayerNotFound = 6201,
    RateLimitExceeded = 6202,
    
    // Treasury errors (6300-6399)
    TreasuryShutdown = 6300,
    PayoutLimitExceeded = 6301,
    InvalidTokenMint = 6302,
}

impl From<CrapsError> for ProgramError {
    fn from(e: CrapsError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
```

### 7.2 Event Replacement Strategy

Since Pinocchio doesn't have Anchor's event system, we use logs:

```rust
pub fn emit_bet_placed(
    player: &Pubkey,
    bet_type: u8,
    amount: u64,
    epoch: u64
) {
    pinocchio::log::sol_log(&format!(
        "BET_PLACED:{}:{}:{}:{}",
        player, bet_type, amount, epoch
    ));
}

// Client-side parsing
const parseLogs = (logs: string[]) => {
    return logs
        .filter(log => log.startsWith("BET_PLACED:"))
        .map(log => {
            const parts = log.split(":");
            return {
                player: parts[1],
                betType: parseInt(parts[2]),
                amount: parseInt(parts[3]),
                epoch: parseInt(parts[4])
            };
        });
};
```

---

## Chapter 8: Testing and Deployment

Testing Pinocchio programs is like testing a race car - you need specialized tools.

### 8.1 Unit Testing with Mollusk

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mollusk_svm::{Mollusk, result::Check};
    
    #[test]
    fn test_place_bet() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "target/deploy/craps_pinocchio");
        
        // Set up accounts
        let player = Pubkey::new_unique();
        let bet_batch = Pubkey::new_unique();
        
        // Create instruction
        let ix_data = PlaceBetData {
            bet_type: BetType::Pass as u8,
            amount: 100u64.to_le_bytes(),
        };
        
        let instruction = Instruction::new_with_bytes(
            program_id,
            &ix_data.try_to_vec()?,
            vec![
                AccountMeta::new(player, false),
                AccountMeta::new(bet_batch, false),
            ],
        );
        
        // Execute and verify
        mollusk.process_and_validate_instruction(
            &instruction,
            &accounts,
            &[
                Check::success(),
                Check::account(&bet_batch)
                    .data(expected_bet_data)
                    .build(),
            ],
        );
    }
}
```

### 8.2 Integration Testing

```rust
#[tokio::test]
async fn test_full_game_flow() {
    let mut test = ProgramTest::new(
        "craps_pinocchio",
        crate::ID,
        processor!(process_instruction)
    );
    
    let mut context = test.start_with_context().await;
    
    // Initialize system
    let tx = Transaction::new_signed_with_payer(
        &[initialize_system_ix()],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await?;
    
    // Test game flow...
}
```

### 8.3 Deployment Script

```bash
#!/bin/bash
# deploy.sh

# Build program
cargo build-sbf --manifest-path Cargo.toml

# Deploy to devnet
solana program deploy \
    --program-id keypair.json \
    --url devnet \
    target/deploy/craps_pinocchio.so

# Verify deployment
solana program show <PROGRAM_ID> --url devnet
```

---

## Chapter 9: TUI Migration

The TUI migration is like translating a book - same story, different language for interacting with the program.

### 9.1 Client Generation

```typescript
// Generate TypeScript client from IDL
import { generateClient } from '@metaplex-foundation/shank-js';

const idl = await generateClient({
    programId: CRAPS_PROGRAM_ID,
    programPath: './target/deploy/craps_pinocchio.so'
});
```

### 9.2 Instruction Builders

```typescript
export class CrapsPinocchioClient {
    constructor(
        private connection: Connection,
        private wallet: Wallet,
        private programId: PublicKey
    ) {}
    
    async placeBet(
        betType: BetType,
        amount: number
    ): Promise<TransactionSignature> {
        const player = await this.getPlayerPDA();
        const betBatch = await this.getBetBatchPDA();
        
        const ix = createPlaceBetInstruction({
            player,
            betBatch,
            globalState: this.globalStatePDA,
            treasury: this.treasuryPDA,
            clock: SYSVAR_CLOCK_PUBKEY,
        }, {
            betType,
            amount: new BN(amount),
        });
        
        return await this.sendTransaction([ix]);
    }
}
```

### 9.3 State Parsing

```typescript
export function parseGlobalGameState(data: Buffer): GlobalGameState {
    return {
        currentEpoch: new BN(data.slice(0, 8), 'le'),
        dice1: data[8],
        dice2: data[9],
        point: data[10],
        phase: data[11] as GamePhase,
        rngAuthority: new PublicKey(data.slice(12, 44)),
        adminAuthority: new PublicKey(data.slice(44, 76)),
        // ... continue parsing
    };
}
```

---

## Appendix: Complete Code Reference

### A.1 Program Structure

```
craps-pinocchio/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Entry point
│   ├── processor.rs        # Instruction router
│   ├── error.rs            # Error definitions
│   ├── constants.rs        # Program constants
│   ├── state/              # Account structures
│   │   ├── mod.rs
│   │   ├── game.rs         # Game state
│   │   ├── player.rs       # Player state
│   │   ├── bet.rs          # Bet structures
│   │   ├── treasury.rs     # Treasury state
│   │   └── rng.rs          # RNG state
│   ├── instructions/       # Instruction handlers
│   │   ├── mod.rs
│   │   ├── betting/        # Bet-related
│   │   ├── game/           # Game flow
│   │   ├── treasury/       # Token operations
│   │   └── admin/          # Admin functions
│   └── utils/              # Helper functions
│       ├── mod.rs
│       ├── bet_encoding.rs
│       ├── pda.rs
│       └── validation.rs
├── tests/
│   ├── integration.rs
│   └── unit.rs
└── client/
    ├── src/
    │   ├── index.ts
    │   ├── instructions.ts
    │   └── state.ts
    └── package.json
```

### A.2 Key Implementation Files

The complete implementation spans multiple files. Here's the organization:

1. **Core State Management**: `/state/` directory
2. **Instruction Processing**: `/instructions/` directory
3. **Utility Functions**: `/utils/` directory
4. **Testing Suite**: `/tests/` directory
5. **Client Library**: `/client/` directory

### A.3 Migration Checklist

- [ ] Set up Pinocchio development environment
- [ ] Migrate all account structures to Pod types
- [ ] Implement instruction discriminators
- [ ] Convert all state management to byte operations
- [ ] Implement bit-packing for bets
- [ ] Migrate RNG system
- [ ] Implement treasury operations
- [ ] Create error handling system
- [ ] Write comprehensive tests
- [ ] Generate TypeScript client
- [ ] Migrate TUI to use new client
- [ ] Deploy and verify on devnet
- [ ] Performance benchmarking
- [ ] Security audit

---

## Troubleshooting: Common Migration Issues

When migrating from Anchor to Pinocchio, you'll likely encounter these issues. Here's how to solve them:

### Issue 1: "My account deserialization is failing"

**Symptom**: `ProgramError::InvalidAccountData` when reading accounts

**Anchor Code That Worked**:
```rust
let player = &mut ctx.accounts.player;
let balance = player.balance;
```

**Pinocchio Fix**:
```rust
// Always check account ownership first!
if account.owner != &crate::ID {
    return Err(ProgramError::IncorrectProgramId);
}

// Check data length
if account.data_len() < Player::LEN {
    return Err(ProgramError::InvalidAccountData);
}

// Now safe to borrow
let data = account.data.borrow();
let player = Player::cast(&data);
```

### Issue 2: "Arithmetic overflow in release mode"

**Symptom**: Program works in tests but fails on-chain

**Problem Code**:
```rust
let new_balance = player.get_balance() + amount; // Can overflow!
```

**Fix**:
```rust
let new_balance = player.get_balance()
    .checked_add(amount)
    .ok_or(CrapsError::Overflow)?;
```

### Issue 3: "PDA derivation mismatch"

**Symptom**: `ProgramError::InvalidSeeds` even though seeds look correct

**Common Mistake**:
```rust
// Anchor automatically includes the bump
let (pda, _) = Pubkey::find_program_address(
    &[b"player", user.as_ref()],
    &program_id
);
```

**Pinocchio Reality**:
```rust
// You must validate the PDA yourself
let (expected_pda, expected_bump) = Pubkey::find_program_address(
    &[b"player", user.as_ref()],
    &crate::ID
);

// Validate it matches
if account.key() != &expected_pda {
    return Err(ProgramError::InvalidSeeds);
}

// If creating, use the bump
let seeds = &[
    b"player",
    user.as_ref(),
    &[expected_bump], // Don't forget this!
];
```

### Issue 4: "Transaction too large"

**Symptom**: Transactions fail with "transaction too large"

**Anchor Hidden Cost**:
```rust
// Anchor adds metadata you don't see
#[derive(Accounts)]
pub struct BigInstruction<'info> {
    // ... 20 accounts
}
```

**Pinocchio Solution**:
```rust
// Split into multiple instructions
pub enum CrapsInstruction {
    InitializePhase1, // First 10 accounts
    InitializePhase2, // Next 10 accounts
}
```

### Issue 5: "Signer validation failed"

**Easy Anchor**:
```rust
#[account(mut, signer)]
pub authority: Signer<'info>,
```

**Manual Pinocchio**:
```rust
// You must check explicitly
if !authority.is_signer {
    return Err(ProgramError::MissingRequiredSignature);
}

// For PDAs signing
let seeds = &[b"treasury", &[bump]];
let signer = [seeds];
invoke_signed(&ix, accounts, &signer)?;
```

### Issue 6: "Account not writable"

**Hidden Anchor Magic**:
```rust
#[account(mut)]
pub player: Account<'info, Player>,
```

**Explicit Pinocchio**:
```rust
// Check before writing
if !account.is_writable {
    return Err(ProgramError::InvalidAccountData);
}

// Also check in instruction definition
#[account(0, writable, name = "player")]
```

### Issue 7: "Clock sysvar access"

**Anchor Convenience**:
```rust
let clock = Clock::get()?;
```

**Pinocchio Method**:
```rust
use pinocchio::sysvars::clock::Clock;

// In your handler
let clock = Clock::get()?; // Still works!

// Or from accounts
let clock_account = &accounts[4];
let clock = Clock::from_account_info(clock_account)?;
```

### Issue 8: "Token program CPI failing"

**Common Error**:
```rust
// Forgot the token program owns token accounts
if token_account.owner != &spl_token::ID {
    return Err(ProgramError::IncorrectProgramId);
}
```

**Correct Pattern**:
```rust
// Validate token account
let token_data = token_account.data.borrow();
let token = TokenAccount::cast(&token_data);

// Check mint matches
if Pubkey::from(token.mint) != expected_mint {
    return Err(CrapsError::InvalidTokenMint.into());
}
```

### Issue 9: "Compute units exceeded"

**Inefficient Approach**:
```rust
// Multiple borrows
for i in 0..10 {
    let data = account.data.borrow();
    // Process...
    drop(data);
}
```

**Efficient Approach**:
```rust
// Single borrow
let data = account.data.borrow();
for i in 0..10 {
    // Process using same borrow
}
```

### Issue 10: "Byte alignment issues"

**Problem**:
```rust
#[repr(C)]
pub struct Misaligned {
    pub byte: u8,      // 1 byte
    pub number: [u8; 8], // Misaligned!
}
```

**Solution**:
```rust
#[repr(C)]
pub struct Aligned {
    pub byte: u8,
    pub _padding: [u8; 7], // Align to 8 bytes
    pub number: [u8; 8],   // Now aligned!
}
```

### Quick Debugging Checklist

When something goes wrong:

1. ✓ Check account ownership
2. ✓ Verify account is writable if modifying
3. ✓ Ensure signers are validated
4. ✓ Validate PDA derivation matches
5. ✓ Check arithmetic for overflows
6. ✓ Verify byte alignment in structures
7. ✓ Ensure proper error propagation
8. ✓ Check compute unit usage
9. ✓ Validate account data length
10. ✓ Test on devnet before mainnet

### Getting Help

- Pinocchio Discord: [discord.gg/pinocchio]
- Example Programs: [github.com/exo-tech-xyz/pinocchio-examples]
- This Guide: Keep it handy!

Remember: Every error is a learning opportunity. Pinocchio makes everything explicit, so when something fails, the error tells you exactly what went wrong.

---

## Conclusion

Migrating from Anchor to Pinocchio is like learning to drive a manual transmission after using automatic - more control, better performance, but requires understanding the mechanics.

The key insights:
1. **Think in bytes, not objects**
2. **Every operation is explicit**
3. **Performance comes from simplicity**
4. **Testing is crucial**

Remember Feynman's approach: "If you can't explain it simply, you don't understand it well enough." This migration guide breaks down complex concepts into understandable pieces, just like Feynman would.

Now, let's implement this step by step...