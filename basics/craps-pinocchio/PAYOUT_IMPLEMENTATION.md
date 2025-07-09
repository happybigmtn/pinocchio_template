# Payout Implementation Status

## Current State

The payout calculation logic is already fully implemented in `src/instructions/claim.rs`. The `calculate_bet_payout` function (line 292) provides comprehensive payout calculations for all 64 bet types.

## Key Functions

### Main Payout Calculator
- `calculate_bet_payout()` - Calculates payouts for all bet types based on:
  - Bet metadata (type and amount)
  - Dice total
  - Game phase
  - Point value
  - Bonus state (for repeater bets)

### Individual Bet Evaluators
The implementation includes specific evaluation functions for each bet category:

1. **Core Bets**:
   - `evaluate_pass_line()` - Pass line bet (1:1)
   - `evaluate_dont_pass()` - Don't pass bet (1:1)
   - `evaluate_come()` - Come bet (1:1)
   - `evaluate_dont_come()` - Don't come bet (1:1)
   - `evaluate_field()` - Field bet (2:1 on 2/12, 1:1 others)

2. **Number Bets**:
   - `evaluate_yes_bet()` - YES bets (various odds)
   - `evaluate_no_bet()` - NO bets (lay odds)
   - `evaluate_next_bet()` - NEXT bets (single roll)

3. **Special Bets**:
   - `evaluate_hard_way()` - Hard way bets (7:1 or 9:1)
   - `evaluate_odds_pass()` - True odds on pass line
   - `evaluate_odds_dont_pass()` - True odds on don't pass
   - `evaluate_hop_bet()` - Hop bets (15:1 or 30:1)
   - `evaluate_any_seven()` - Any seven (4:1)
   - `evaluate_any_craps()` - Any craps (7:1)

4. **Advanced Bets**:
   - `evaluate_repeater()` - Repeater bets (10:1 to 100:1)
   - `evaluate_ride_line()` - Ride the line
   - `evaluate_muggsy()` - Muggsy bet

## Integration Points

The payout logic is used in:
1. `claim_epoch_payouts_unified_handler()` - Main claim instruction
2. Bet settlement during epoch resolution
3. Treasury balance calculations

## No Additional Implementation Needed

Since the payout calculation logic is already fully implemented and tested in the claim module, no additional payout calculator needs to be created. The existing implementation:

- ✅ Handles all 64 bet types
- ✅ Calculates correct odds for each bet
- ✅ Considers game phase and point
- ✅ Integrates with bonus state for repeater bets
- ✅ Returns proper payout amounts in lamports

## Usage

To calculate payouts, the system:
1. Decodes bet data from the bet batch
2. Retrieves epoch outcome (dice, phase, point)
3. Calls `calculate_bet_payout()` for each bet
4. Aggregates total payout
5. Transfers tokens from treasury to player

The payout logic is production-ready and does not need modification.