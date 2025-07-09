# Bet Encoding Implementation Summary

This document summarizes the bet encoding utilities and constants implemented for craps-pinocchio.

## Files Created/Modified

### 1. `/src/utils/mod.rs`
- Created module structure for utilities
- Exports bet_encoding and validation modules

### 2. `/src/utils/bet_encoding.rs`
Key functions implemented:
- `encode_bet(bet_type: u8, amount: u64) -> Result<u16, ProgramError>` - Packs bet type (6 bits) and amount index (10 bits) into u16
- `decode_bet(packed: u16) -> (u8, u64)` - Unpacks bet data to get type and amount
- `encode_amount(amount: u64) -> Result<u16, ProgramError>` - Non-linear encoding of amounts:
  - 1-100 CRAP: Direct mapping (indices 0-99)
  - 101-500 CRAP by 5s (indices 100-179)
  - 501-1500 CRAP by 10s (indices 180-279)
  - 1501-5000 CRAP by 25s (indices 280-419)
  - 5001-10000 CRAP by 50s (indices 420-519)
  - 10001-20000 CRAP by 100s (indices 520-619)
  - 20001-40000 CRAP by 250s (indices 620-699)
  - 40001-60000 CRAP by 500s (indices 700-739)
  - 60001-80000 CRAP by 1000s (indices 740-759)
  - 80001-100000 CRAP by 2500s (indices 760-767)
- `decode_amount(index: u16) -> u64` - Decodes amount index back to lamports
- `decode_amount_safe(index: u16) -> Result<u64, ProgramError>` - Decode with validation

### 3. `/src/utils/validation.rs`
Validation helpers implemented:
- `validate_bet_type(bet_type: u8)` - Validates bet type is within 0-63 range
- `validate_dice_roll(die1: u8, die2: u8)` - Validates dice values and returns total
- `validate_bet_for_phase(bet_type: u8, game_phase: u8, current_point: u8)` - Ensures bet is valid for current game phase
- `validate_bet_amount(amount: u64)` - Validates amount is within min/max limits
- `validate_deposit_amount(amount: u64)` - Validates deposit amounts
- `validate_withdrawal_amount(amount: u64, balance: u64)` - Validates withdrawals
- Helper functions for bet type identification:
  - `is_valid_point(num: u8)`
  - `is_natural(total: u8)`
  - `is_craps(total: u8)`
  - `is_hard_way(die1: u8, die2: u8)`
  - `is_yes_bet(bet_type: u8)`
  - `is_no_bet(bet_type: u8)`
  - `is_next_bet(bet_type: u8)`
  - `is_repeater_bet(bet_type: u8)`
  - `is_odds_bet(bet_type: u8)`
  - `is_bonus_bet(bet_type: u8)`
  - `is_multi_roll_bet(bet_type: u8)`
- Target number getters for different bet types

### 4. `/src/constants.rs`
Updated with all constants from the Anchor version:
- **PDA Seeds**: All 15 PDA seed constants
- **Bet Type Constants**: All 64 bet types (BET_PASS through BET_REPEATER_12)
- **Game Phase Constants**: PHASE_COME_OUT, PHASE_POINT
- **Game Constants**: Dice values, naturals, craps numbers, valid points
- **Limits**: Bet amounts, deposits, withdrawals, daily limits
- **RNG Constants**: Block hashes, roll intervals, betting windows
- **Treasury Constants**: Safety multipliers, minimum balance, commission
- **Batch Constants**: Max bets per batch, active batches
- **Financial Constants**: Basis points, percentages

### 5. `/src/error.rs`
Replaced with comprehensive error enum mapping to Pinocchio's ProgramError:
- 162 error codes organized in ranges:
  - 0-99: General/System errors
  - 100-199: Core game errors
  - 200-299: Social errors (reserved)
  - 300-399: Tournament errors (reserved)

### 6. `/src/lib.rs`
- Added `pub mod utils;` to include the new utilities module

### 7. `/src/utils/test_encoding.rs`
- Comprehensive test suite for bet encoding/decoding
- Tests for all encoding ranges, invalid amounts, and edge cases
- Written to be no_std compatible (using arrays instead of vec!)

## Key Features

1. **Efficient Storage**: Bets are packed into 16 bits (6 for type, 10 for amount)
2. **Non-linear Encoding**: Supports amounts from 0.001 to 100,000 CRAP tokens with appropriate granularity
3. **Comprehensive Validation**: All bet types, amounts, and game states are validated
4. **No-std Compatible**: All code works in Solana's no_std environment
5. **Complete Error Handling**: All Anchor error codes mapped to Pinocchio

## Usage Example

```rust
use craps_pinocchio::utils::{encode_bet, decode_bet, validate_bet_type, validate_bet_for_phase};
use craps_pinocchio::constants::{BET_PASS, PHASE_COME_OUT};

// Encode a Pass bet of 100 CRAP
let bet_type = BET_PASS;
let amount = 100_000_000_000; // 100 CRAP in lamports
let packed = encode_bet(bet_type, amount)?;

// Decode it back
let (decoded_type, decoded_amount) = decode_bet(packed);
assert_eq!(decoded_type, BET_PASS);
assert_eq!(decoded_amount, amount);

// Validate bet for current phase
validate_bet_for_phase(BET_PASS, PHASE_COME_OUT, 0)?;
```

The implementation is complete and ready for integration with the rest of the craps-pinocchio program.